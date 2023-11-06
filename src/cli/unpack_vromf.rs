use clap::{Arg, Command, ValueHint};

pub fn unpack_vromf() -> Command {
	Command::new("unpack_vromf")
		.long_flag("unpack_vromf")
		.about("Unpacks vromf into raw or human readable formats, such as Json or Blk")
		.arg_required_else_help(true)
		.arg(
			Arg::new("Input file/directory")
				.short('i')
				.long("input_dir_or_file")
				.help("A single vromf file, or a folder of Vromf files. Does not recurse subdirs")
				.required(true)
				.value_hint(ValueHint::AnyPath)
		)
		.arg(
			// Not providing this argument means the input folder name will be used, with a `_u` suffix
			Arg::new("Output directory")
				.short('o')
				.long("output_dir")
				.help("Target folder that will be created to contain new files")
				.value_hint(ValueHint::FilePath)
		)
		.arg(
			Arg::new("format")
				.long("format")
				.help("Output format, can be one of: [Json, BlkText, Raw]")
				.default_value("Json")
		)
		.arg(
			Arg::new("override")
				.long("override")
				.help("Applies `override:` fields in every json")
				.num_args(0)
				.required(false)
		)
		.arg(
			Arg::new("avif2dds")
				.long("avif2dds")
				.help("Converts all avif images to dds")
				.num_args(0)
				.required(false)
		)
		.arg(
			Arg::new("crlf")
				.long("crlf")
				.num_args(0) // expects no values
				.required(false)
				.help("Returns files with \\r\\n instead of \\n newlines")
		)
}