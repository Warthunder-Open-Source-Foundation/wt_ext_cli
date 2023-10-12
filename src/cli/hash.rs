use clap::Command;

pub fn hash() -> Command {
	Command::new("hash")
		.long_flag("hash")
		.about("Prints commit hash from binary (link)")
}