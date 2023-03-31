use clap::{Arg, ColorChoice, Command, command};

pub fn build_command_structure() -> Command {
	let matches = command!("wt_ext_cli")
		.about("WarThunder datamining extraction tools")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.color(ColorChoice::Always)
		.author("FlareFlo")
		.arg(
			Arg::new("log_path")
				.long("log_path")
				.help("When provided, writes the traced logs to a file")
		)
		.arg(
			Arg::new("log_level")
				.long("log_level")
				.help("Set log level, may be one of [Trace, Debug, Info, Warn, Error], default: Warn")
		)
		.subcommand(
			Command::new("unpack_raw_blk")
				.long_flag("unpack_raw_blk")
				.about("Unpacks a folder of raw/binary blk files into their unpacked format")
				.arg(
					Arg::new("Input directory")
						.short('i')
						.long("input_dir")
						.help("Folder containing blk files, sub-folders will be recursively searched")
						.required(true)
				)
				.arg(
					// Not providing this argument means the input folder name will be used, with a `_u` suffix
					Arg::new("Output directory")
						.short('o')
						.long("output_dir")
						.help("Target folder that will be created to contain new files")
						.conflicts_with("Overwrite")
				)
				.arg(
					Arg::new("Overwrite")
						.help("Overwrites binary BLk files in input folder")
						.conflicts_with("Output directory")
				)
		)
		.subcommand(
			Command::new("unpack_vromf NYIMPL")
				.long_flag("unpack_vromf")
				.about("Unpacks vromf into raw or human readable formats, such as Json or Blk")
				.arg_required_else_help(true)
				.arg(
					Arg::new("format")
						.help("Output format, can be one of: [Json, BlkText, BlkRaw]")
						.required(true)
				)
		)
		.subcommand(
			Command::new("diff_yup NYIMPL")
				.long_flag("diff_yup")
				.about("Creates diff from .yup")
		)
		.subcommand(
			Command::new("update_check NYIMPL")
				.long_flag("check_update")
				.about("Checks folder for client update")
		)
		;

	matches
}
