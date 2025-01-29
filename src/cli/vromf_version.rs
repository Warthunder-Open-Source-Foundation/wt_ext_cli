use clap::{Arg, Command, ValueHint};

pub fn vromf_version() -> Command {
	Command::new("vromf_version")
		.long_flag("vromf_version")
		.arg(
			Arg::new("input")
				.short('i')
				.long("input_dir_or_file")
				.help("A single vromf file, or a folder of Vromf files. Does not recurse subdirs")
				.required(true)
				.value_hint(ValueHint::AnyPath),
		)
		.arg(
			Arg::new("format")
				.short('f')
				.long("format")
				.help("Prints the version either in plain or json text format")
				.default_value("json"),
		)
		.about("Prints version(s) from file or folder of vromfs")
}
