use clap::Command;

pub fn diff_yup() -> Command {
	Command::new("diff_yup")
		.long_flag("diff_yup")
		.about("Creates diff from .yup")
		.hide(true)
}
