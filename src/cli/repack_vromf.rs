use clap::{Arg, ArgAction, Command, ValueHint};

pub fn repack_vromf() -> Command {
	Command::new("repack_vromf")
		.long_flag("repack_vromf")
		.about("Packs files from a directory into a VROMFS archive (or round-trips an existing vromf)")
		.arg_required_else_help(true)
		.arg(
			Arg::new("input")
				.short('i')
				.long("input")
				.help("Input: a directory of files (fresh pack) or an existing .vromfs.bin file (round-trip repack)")
				.required(true)
				.value_hint(ValueHint::AnyPath),
		)
		.arg(
			Arg::new("output")
				.short('o')
				.long("output")
				.help("Output .vromfs.bin file path")
				.required(true)
				.value_hint(ValueHint::FilePath),
		)
		.arg(
			Arg::new("header")
				.long("header")
				.help("Header format: vrfs (base) or vrfx (extended). Auto-detected from existing metadata when available")
				.num_args(1),
		)
		.arg(
			Arg::new("platform")
				.long("platform")
				.help("Target platform: pc, ios, android. Auto-detected from existing metadata when available")
				.num_args(1),
		)
		.arg(
			Arg::new("packing")
				.long("packing")
				.help("Packing/compression: plain, zstd_obfs, or zstd_obfs_nocheck. Auto-detected from existing metadata when available")
				.num_args(1),
		)
		.arg(
			Arg::new("version")
				.long("version")
				.help("Game version for VRFX headers (e.g. 2.33.1.0). Required when creating a new VRFX header without existing metadata")
				.num_args(1),
		)
		.arg(
			Arg::new("digest")
				.long("digest")
				.num_args(0)
				.required(false)
				.action(ArgAction::SetTrue)
				.help("Include per-file SHA1 checksums. Auto-detected when possible, defaults to false otherwise"),
		)
}
