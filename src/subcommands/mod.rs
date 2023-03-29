use clap::ArgMatches;
use crate::subcommands::unpack_raw_blk::unpack_raw_blk;

mod unpack_raw_blk;

pub fn branch_subcommands(args: ArgMatches) {
	match args.subcommand() {
		Some(("unpack_raw_blk", args)) => {
			unpack_raw_blk(args).unwrap();
		}
		_ => {
			panic!("Ruh oh, looks like command args were bad");
		}
	}
}