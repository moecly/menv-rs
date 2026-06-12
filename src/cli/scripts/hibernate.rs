use std::path::PathBuf;

use color_eyre::eyre::{Result, bail};

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

    let output = std::process::Command::new("du")
        .arg("-h")
        .arg(swapfile)
        .output()?;

    if !output.status.success() {
        bail!("{}", str::from_utf8(&output.stderr)?)
    }

    println!("{}", str::from_utf8(&output.stdout)?);

    Ok(())
}
