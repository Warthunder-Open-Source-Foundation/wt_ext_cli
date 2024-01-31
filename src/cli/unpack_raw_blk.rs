use clap::{Arg, Command, ValueHint};

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
				.value_hint(ValueHint::FilePath),
		)
		.arg(
			// Not providing this argument means the input folder name will be used, with a `_u` suffix
			Arg::new("Output directory")
				.short('o')
				.long("output_dir")
				.help("Target folder that will be created to contain new files")
				.value_hint(ValueHint::FilePath),
		)
		.arg(
			Arg::new("format")
				.long("format")
				.help("Output format, can be one of: [Json, BlkText]")
				.default_value("Json")
		)
}
