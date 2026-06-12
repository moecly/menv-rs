use color_eyre::eyre::{Result, bail};

use crate::cli::common::Common;

#[derive(Debug)]
pub struct Update;

impl Update {
    pub fn handle_cmd() -> Result<()> {
        Self::update()
    }

    fn update() -> Result<()> {
        Common::print_emoji_title("🔄", "Updating menv-rs");

        let bin_path = Common::get_bin_path()?;
        let updater_path = bin_path.join("menv-rs-update");
        if !updater_path.exists() {
            Common::print_error(format!("menv-rs-update not found: {:?}", updater_path).as_str());
            Common::print_error("Please reinstall menv-rs using the install script");
            bail!("menv-rs-update not exists")
        }

        let status = std::process::Command::new(updater_path).status()?;
        if status.success() {
            Common::print_progress_done("Update completed");
            Common::print_success("menv-rs has been updated to the latest version");
        } else {
            Common::print_progress_failed(
                "Update failed",
                &format!("exit code: {:?}", status.code()),
            );
            Common::print_error("Please try again later or manually download the latest version");
            bail!("update failed");
        }
        Ok(())
    }
}
