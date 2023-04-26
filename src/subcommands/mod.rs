use std::{fs, str::FromStr};

use clap::ArgMatches;
use tracing::{metadata::LevelFilter};


use crate::{
	logging::init_logging,
	subcommands::{
		unpack_dxp::unpack_dxp,
		unpack_raw_blk::unpack_raw_blk,
		unpack_vromf::unpack_vromf,
	},
};

mod unpack_dxp;
mod unpack_raw_blk;
mod unpack_vromf;

pub fn branch_subcommands(args: ArgMatches) -> Result<(), anyhow::Error> {
	let log_level = if let Some(lvl) = args.get_one::<String>("log_level") {
		LevelFilter::from_str(lvl).expect(&format!("Incorrect log-level provided, expected one of [Trace, Debug, Info, Warn, Error], found {lvl}"))
	} else {
		LevelFilter::WARN
	};
	let _file_writer = if let Some(log_path) = args.get_one::<String>("log_path") {
		Some(fs::File::create(log_path)?)
	} else {
		None
	};
	init_logging(log_level);

	match args.subcommand() {
		Some(("unpack_raw_blk", args)) => {
			unpack_raw_blk(args)?;
		},
		Some(("unpack_vromf", args)) => {
			unpack_vromf(args)?;
		},
		Some(("unpack_dxp", args)) => {
			unpack_dxp(args)?;
		},
		Some(("get_instruction_manual", _)) => {
			open::that("https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/blob/master/usage_manual.md").expect("Attempted to show manual in browser, but something unexpected failed");
		},
		_ => {
			panic!("Ruh oh, looks like command args were bad");
		},
	}
	Ok(())
}
