use std::{fs, path::PathBuf, process::Command};

use color_eyre::eyre::{ContextCompat, Result, bail};
use regex::Regex;

use crate::cli::{common::Common, tools::Tools};

pub fn run() -> Result<()> {
    let swapfile = PathBuf::from("/swap/swapfile");
    let mkinitcpio_dropin = PathBuf::from("/etc/mkinitcpio.conf.d/resume.conf");
    let cmdline = PathBuf::from("/etc/kernel/cmdline");

    if !Tools::cmd_v("bootctl")? {
        Common::print_error("bootctl not found");
        Common::print_error("Please install systemd-boot first");
        bail!("bootctl not found");
    }

    if !swapfile.exists() || !mkinitcpio_dropin.exists() || !cmdline.exists() {
        bail!("error")
    }

    let swap_size = Common::process_command(Command::new("du").arg("-h").arg(&swapfile))?;
    println!("{}", swap_size);

    let uuid = Common::process_command(
        Command::new("findmnt")
            .arg("-no")
            .arg("UUID")
            .arg("-T")
            .arg(&swapfile),
    )?
    .trim()
    .to_string();

    println!("{}", uuid);

    let offset = Common::process_command(
        Command::new("sudo")
            .arg("btrfs")
            .arg("inspect-internal")
            .arg("map-swapfile")
            .arg(&swapfile),
    )?;

    println!("{}", offset);
    let mut offset_val: usize = 0;
    for s in offset.lines() {
        if !s.starts_with("Resume offset:") {
            continue;
        }
        offset_val = s
            .strip_prefix("Resume offset:")
            .context("strip_prefix failed")?
            .trim_start()
            .parse()?;
    }
    println!("{}", offset_val);

    let resume_param = format!("resume=UUID={}", uuid);
    let offset_param = format!("resume_offset={}", offset_val);
    println!("{} {}", resume_param, offset_param);

    let resume_str = fs::read_to_string(mkinitcpio_dropin)?;

    let re = Regex::new(r"HOOKS\s*\+=\s*\(resume\)")?;
    let hook_exists = re.is_match(&resume_str);

    Ok(())
}
