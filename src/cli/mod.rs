use clap::{command, Arg, ArgAction, ColorChoice, Command, ValueHint};

pub fn build_command_structure() -> Command {
	let matches = command!("wt_ext_cli")
		.about("WarThunder datamining extraction tools")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.color(ColorChoice::Auto)
		.author("FlareFlo")
		.arg(
			Arg::new("log_path")
				.long("log_path")
				.help("When provided, writes the traced logs to a file")
				.value_hint(ValueHint::FilePath)
		)
		.arg(
			Arg::new("log_level")
				.long("log_level")
				.help("Set log level, may be one of [Trace, Debug, Info, Warn, Error], default: Warn")
		)
		.arg(
			Arg::new("crashlog")
				.long("crashlog")
				.required(false)
				.num_args(0)
				.help("Set this to run at maximum log level to aid in debugging")
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
						.value_hint(ValueHint::FilePath)
				)
				.arg(
					// Not providing this argument means the input folder name will be used, with a `_u` suffix
					Arg::new("Output directory")
						.short('o')
						.long("output_dir")
						.help("Target folder that will be created to contain new files")
						.conflicts_with("Overwrite")
						.value_hint(ValueHint::FilePath)
				)
				.arg(
					Arg::new("Overwrite")
						.long("overwrite")
						.num_args(0) // expects no values
						.help("Overwrites files in input folder")
						.conflicts_with("Output directory")
				)
		)
		.subcommand(
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
						.help("Output format, can be one of: [Json, BlkText, Raw]. Defaults to Json")
						.default_value("Json")
				)
				.arg(
					Arg::new("crlf")
						.long("crlf")
						.num_args(0) // expects no values
						.required(false)
						.help("Returns files with \r\n instead of \n newlines")
				)
		)
		.subcommand(
			Command::new("unpack_dxp_and_grp")
				.long_flag("unpack_dxp")
				.about("Unpacks folder and subfolder DXP and GRP files to text-formatted file")
				.arg(
					Arg::new("Input directory")
						.short('i')
						.long("input_dir")
						.help("Folder with DXP/GRP files inside")
						.required(true)
						.value_hint(ValueHint::AnyPath)
				)
				.arg(
					Arg::new("Output directory")
						.short('o')
						.long("output_dir")
						.help("Target folder that will be created to contain new files, preserving file structure")
						.value_hint(ValueHint::FilePath)
				)
				.arg(
					Arg::new("Keep suffix")
						.long("keep_suffix")
						.help("Paths and Names inside the final DXP/GRP are always followed by \"water_garbage_pile_b_tex_d$hq*\" or random unicode chars \"u+4575\"")
						.num_args(0) // Expects only a flag, no data
						.action(ArgAction::SetTrue)
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
		.subcommand(
			Command::new({
				pub const COMMAND_MANUAL: &str = "get_instruction_manual";
				COMMAND_MANUAL
			})
				.long_flag("instruction_manual")
				.about("Opens or writes the manual")
		)
		;

	matches
}
