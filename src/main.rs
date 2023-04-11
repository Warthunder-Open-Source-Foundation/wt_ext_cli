#![feature(if_let_guard)]

use std::{fs, thread};
use std::fs::ReadDir;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;
use wt_blk::binary::{DecoderDictionary, parse_file};
use wt_blk::binary::nm_file::NameMap;

use crate::cli::build_command_structure;
use crate::fs_util::find_dict;
use crate::logging::init_logging;
use crate::subcommands::branch_subcommands;

mod fs_util;
mod update_diff;
mod cli;
mod logging;
mod subcommands;
mod error;
mod task_queue;
mod util;


fn main() -> Result<(), anyhow::Error> {
	let command = build_command_structure().get_matches();
	branch_subcommands(command)?;

	Ok(())
}