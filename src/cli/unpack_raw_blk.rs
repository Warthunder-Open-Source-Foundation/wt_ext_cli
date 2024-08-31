use clap::{Arg, ArgAction::SetTrue, Command, ValueHint};

pub fn unpack_raw_blk() -> Command {
	Command::new("unpack_raw_blk")
		.long_flag("unpack_raw_blk")
		.about("Unpacks a folder of raw/binary blk files into their unpacked format")
		.arg(
			Arg::new("Input directory")
				.short('i')
				.long("input_dir")
				.help("Folder containing blk files, sub-folders will be recursively searched")
				.required(true)
				.value_hint(ValueHint::FilePath)
				.conflicts_with("stdin"),
		)
		.arg(
			// Not providing this argument means the input folder name will be used, with a `_u` suffix
			Arg::new("Output directory")
				.short('o')
				.long("output_dir")
				.help("Target folder that will be created to contain new files")
				.value_hint(ValueHint::FilePath)
				.conflicts_with("stdout"),
		)
		.arg(
			Arg::new("Name map")
				.long("nm")
				.help("Path to name map")
				.value_hint(ValueHint::FilePath)
		)
		.arg(
			Arg::new("stdout")
				.long("stdout")
				.help("writes to stdout instead of a file")
				.action(SetTrue),
		)
		.arg(
			Arg::new("stdin")
				.long("stdin")
				.help("reads from stdin instead of a file")
				.action(SetTrue),
		)
		.arg(
			Arg::new("format")
				.long("format")
				.help("Output format, can be one of: [Json, BlkText]")
				.default_value("Json"),
		)
}
