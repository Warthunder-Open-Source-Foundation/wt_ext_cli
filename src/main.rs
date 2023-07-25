#![feature(if_let_guard)]

use std::env;

use color_eyre::eyre::Result;

use crate::{cli::build_command_structure, subcommands::branch_subcommands};

mod cli;
mod error;
mod fs_util;
mod logging;
mod subcommands;
mod update_diff;
mod util;

fn main() -> Result<()> {
	env::set_var("RUST_BACKTRACE", "1");
	color_eyre::install()?;

	let command = build_command_structure().get_matches();
	branch_subcommands(command)?;

	Ok(())
}
