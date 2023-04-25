#![feature(if_let_guard)]

use std::{
	fs,
	fs::ReadDir,
	rc::Rc,
	sync::{atomic::AtomicUsize, Arc},
	thread,
};

use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;
use wt_blk::binary::{nm_file::NameMap, parse_file, DecoderDictionary};

use crate::{
	cli::build_command_structure,
	fs_util::find_dict,
	logging::init_logging,
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
