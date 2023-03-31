use std::str::FromStr;
use clap::ArgMatches;
use tracing::Level;
use tracing::metadata::LevelFilter;
use tracing_subscriber::filter::Directive;
use crate::logging::init_logging;
use crate::subcommands::unpack_raw_blk::unpack_raw_blk;

mod unpack_raw_blk;

pub fn branch_subcommands(args: ArgMatches) {
	let log_level = if let Some(lvl) = args.get_one::<String>("log_level") {
		LevelFilter::from_str(lvl).expect(&format!("Incorrect log-level provided, expected one of [Trace, Debug, Info, Warn, Error], found {lvl}"))
	} else {
		LevelFilter::WARN
	};
	init_logging(log_level);

	match args.subcommand() {
		Some(("unpack_raw_blk", args)) => {
			unpack_raw_blk(args).unwrap();
		}
		_ => {
			panic!("Ruh oh, looks like command args were bad");
		}
	}
}