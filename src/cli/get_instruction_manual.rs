use clap::Command;

pub fn get_instruction_manual() -> Command {
	Command::new("get_instruction_manual")
		.long_flag("instruction_manual")
		.about("Opens or writes the manual")
}
