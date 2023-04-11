use std::fs;
use std::str::FromStr;
use clap::ArgMatches;
use tracing::Level;
use tracing::metadata::LevelFilter;
use tracing_subscriber::filter::Directive;
use crate::logging::init_logging;
use crate::subcommands::unpack_raw_blk::unpack_raw_blk;
use crate::subcommands::unpack_vromf::unpack_vromf;

mod unpack_raw_blk;
mod unpack_vromf;

pub fn branch_subcommands(args: ArgMatches) {
	let log_level = if let Some(lvl) = args.get_one::<String>("log_level") {
		LevelFilter::from_str(lvl).expect(&format!("Incorrect log-level provided, expected one of [Trace, Debug, Info, Warn, Error], found {lvl}"))
	} else {
		LevelFilter::WARN
	};
	let file_writer = if let Some(log_path) = args.get_one::<String>("log_path") {
		let file = match fs::File::create(log_path) {
			Ok(fd) => {fd}
			Err(e) => {
				panic!("Failed to write log file, reason: {e}");
			}
		};
		Some(file)
	} else {
		None
	};
	init_logging(log_level);

	match args.subcommand() {
		Some(("unpack_raw_blk", args)) => {
			unpack_raw_blk(args).unwrap();
		}
		Some(("unpack_vromf", args)) => {
			unpack_vromf(args).unwrap();
		}
		Some(("get_instruction_manual", _)) => {
			open::that("https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/blob/master/usage_manual.md").expect("Attempted to show manual in browser, but something unexpected failed");
		}
		_ => {
			panic!("Ruh oh, looks like command args were bad");
		}
	}
}