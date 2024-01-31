use clap::Command;

pub fn update_check() -> Command {
	Command::new("update_check")
		.long_flag("check_update")
		.about("Checks folder for client update")
		.hide(true)
}
