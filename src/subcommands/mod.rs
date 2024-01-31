use std::{str::FromStr, sync::OnceLock};

use clap::ArgMatches;
use color_eyre::eyre::{bail, Result};
use tracing::{error, metadata::LevelFilter};

use crate::{
	logging::init_logging,
	subcommands::{
		unpack_dxp_and_grp::unpack_dxp_and_grp,
		unpack_raw_blk::unpack_raw_blk,
		unpack_vromf::unpack_vromf,
		vromf_version::vromf_version,
	},
	COMMIT_HASH,
};

mod unpack_dxp_and_grp;
mod unpack_raw_blk;
pub mod unpack_vromf;
mod vromf_version;

pub static CRASHLOG: OnceLock<bool> = OnceLock::new();

pub fn branch_subcommands(args: ArgMatches) -> Result<()> {
	// Specific option to run and log everything for debugging
	let crashlog = args.get_flag("crashlog");
	CRASHLOG
		.set(crashlog)
		.expect("Failed to set CRASHLOG global flag");

	let log_level = if let Some(lvl) = args.get_one::<String>("log_level") {
		LevelFilter::from_str(lvl).expect(&format!("Incorrect log-level provided, expected one of [Trace, Debug, Info, Warn, Error], found {lvl}"))
	} else {
		LevelFilter::WARN
	};
	init_logging(
		log_level,
		args.get_one::<String>("log_path").cloned(),
		crashlog,
	)?;

	match args.subcommand() {
		Some(("unpack_raw_blk", args)) => {
			unpack_raw_blk(args)?;
		},
		Some(("unpack_vromf", args)) => {
			unpack_vromf(args)?;
		},
		Some(("unpack_dxp_and_grp", args)) => {
			unpack_dxp_and_grp(args)?;
		},
		Some(("get_instruction_manual", _)) => {
			open::that("https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/blob/master/usage_manual.md").expect("Attempted to show manual in browser, but something unexpected failed");
		},
		Some(("hash", _)) => {
			println!(
				"https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/commit/{}",
				COMMIT_HASH
			);
		},
		Some(("vromf_version", args)) => {
			vromf_version(args)?;
		},
		_ => {
			error!("Unmatched subcommand: {:?}", args.subcommand());
			if let Some((command, _)) = args.subcommand() {
				bail!("Unmatched subcommand: {:}", command)
			} else {
				bail!("Missing Subcommand argument")
			}
		},
	}
	Ok(())
}
