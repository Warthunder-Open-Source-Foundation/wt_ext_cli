mod diff_yup;
mod get_instruction_manual;
mod hash;
mod unpack_dxp_and_grp;
mod unpack_raw_blk;
mod unpack_vromf;
mod update_check;
mod vromf_version;

use clap::{command, Arg, ColorChoice, Command, ValueHint};
use const_format::formatcp;

pub fn build_command_structure() -> Command {
	command!("wt_ext_cli")
		.version(formatcp!("{} {}", crate::GIT_TAG, crate::COMMIT_HASH))
		.about("WarThunder datamining extraction tools")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.color(ColorChoice::Auto)
		.author("FlareFlo")
		.arg(
			Arg::new("log_path")
				.long("log_path")
				.help("When provided, writes the traced logs to a file")
				.value_hint(ValueHint::FilePath),
		)
		.arg(
			Arg::new("log_level").long("log_level").help(
				"Set log level, may be one of [Trace, Debug, Info, Warn, Error], default: Warn",
			),
		)
		.arg(
			Arg::new("crashlog")
				.long("crashlog")
				.required(false)
				.num_args(0)
				.help("Runs at maximum log level and writes logfile to aid in debugging"),
		)
		.subcommand(unpack_raw_blk::unpack_raw_blk())
		.subcommand(unpack_vromf::unpack_vromf())
		.subcommand(unpack_dxp_and_grp::unpack_dxp_and_grp())
		.subcommand(diff_yup::diff_yup())
		.subcommand(update_check::update_check())
		.subcommand(get_instruction_manual::get_instruction_manual())
		.subcommand(hash::hash())
		.subcommand(vromf_version::vromf_version())
}
