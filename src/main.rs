#![feature(if_let_guard)]







use crate::{
	cli::build_command_structure,
	subcommands::branch_subcommands,
};

mod cli;
mod error;
mod fs_util;
mod logging;
mod subcommands;
mod task_queue;
mod update_diff;
mod util;

fn main() -> Result<(), anyhow::Error> {
	let command = build_command_structure().get_matches();
	branch_subcommands(command)?;

	Ok(())
}
