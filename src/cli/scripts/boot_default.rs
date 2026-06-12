use std::{fs, io, path::PathBuf};

use color_eyre::eyre::{Result, bail};

use crate::cli::{common::Common, tools::Tools};

pub fn run() -> Result<()> {
    Common::print_emoji_title("🔧", "Boot Default Configuration");
    if !Tools::cmd_v("bootctl")? {
        Common::print_error("bootctl not found");
        Common::print_error("Please install systemd-boot first");
        bail!("bootctl not found");
    }
    let boot_conf_path = PathBuf::from("/boot/loader/entries");
    if !boot_conf_path.exists() {
        Common::print_error(&format!(
            "Boot entries directory not found: {}",
            boot_conf_path.display()
        ));
        bail!("boot conf path not found");
    }

    let entries: Vec<String> = fs::read_dir(boot_conf_path)?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|e| e.ends_with(".conf"))
        .collect();

    if entries.is_empty() {
        Common::print_success("No boot entries found");
        return Ok(());
    }

    Common::print_success(&format!("Found {} boot entries:", entries.len()));
    entries.iter().enumerate().for_each(|(idx, e)| {
        Common::print_msg(format!("{}. {}", idx, e).as_str());
    });

    Common::print_msg("Enter the number of the boot entry to set as default");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let selected_entry: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            Common::print_error(&format!("Invalid number: {}", input.trim()));
            bail!("Invalid input");
        }
    };
    if selected_entry >= entries.len() {
        Common::print_error(&format!(
            "Invalid selection: {} (max: {})",
            selected_entry,
            entries.len() - 1
        ));
        bail!("Selection out of range");
    }
    Common::print_progress(&format!("Setting default boot entry: {}", selected_entry));

    let output = std::process::Command::new("sudo")
        .arg("bootctl")
        .arg("set-default")
        .arg(&entries[selected_entry])
        .output()?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Common::print_progress_failed("Failed", &err_msg);
        Common::print_error("Failed to set default boot entry");
        bail!("bootctl set-default failed: {}", err_msg);
    } else {
        Common::print_progress_done("Success");
        println!();
        Common::print_success(&format!(
            "Default boot entry set to: {}",
            &entries[selected_entry]
        ));
    }

    Ok(())
}
