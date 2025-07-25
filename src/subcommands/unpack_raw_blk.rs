use std::{
	fs,
	fs::OpenOptions,
	io,
	io::{Read, Write},
	path::{Path, PathBuf},
	str::FromStr,
	sync::Arc,
};

use atty::Stream;
use clap::ArgMatches;
use color_eyre::eyre::{bail, ContextCompat, Result};
use wt_blk::{
	blk,
	blk::{file::FileType, name_map::NameMap},
};

pub fn unpack_raw_blk(args: &ArgMatches) -> Result<()> {
	let format = args
		.get_one::<String>("format")
		.context("Invalid format specified or missing")?;

	let mut input_path = None;
	let mut read = get_input(&args, &mut input_path)?;

	let zstd_dict = None;
	let nm = args
		.get_one::<String>("Name map")
		.map(fs::read)
		.transpose()?
		.map(|e| NameMap::from_encoded_file(&e))
		.transpose()?
		.map(Arc::new);

	let bye_bye = |format| {
		bail!("{format} is not implemented yet. If you need it, poke me with an issue at: https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/issues")
	};
	match FileType::from_byte(read[0])? {
		FileType::BBF => {},
		FileType::FAT => {},
		FileType::FAT_ZSTD => {},
		FileType::SLIM => {},
		FileType::SLIM_ZSTD => {},
		FileType::SLIM_ZST_DICT => {
			bye_bye("ZSTD dictionary")?;
		},
	}

	let mut parsed = blk::unpack_blk(&mut read, zstd_dict, nm)?;

	match format.as_str() {
		"Json" => {
			parsed.merge_fields()?;
			write_output(args, parsed.as_serde_json()?, input_path, format)?;
		},
		"BlkText" => {
			write_output(args, parsed.as_blk_text()?.into_bytes(), input_path, format)?;
		},
		_ => {
			panic!("Unrecognized format: {format}")
		},
	}

	Ok(())
}

pub fn get_input(args: &ArgMatches, input_path: &mut Option<PathBuf>) -> Result<Vec<u8>> {
	if let Some(p) = args.get_one::<String>("Input directory") {
		let input = Path::new(p);
		if input.is_dir() {
			bail!("Directories as input are not implemented yet");
		}
		*input_path = Some(input.to_path_buf());
		return Ok(fs::read(p)?);
	}

	if Some(&true) == args.get_one::<bool>("stdin") {
		if atty::is(Stream::Stdin) {
			bail!("Stdin is not connected!");
		}

		let mut buf = vec![];
		io::stdin().read_to_end(&mut buf)?;
		return Ok(buf);
	}

	bail!("No input passed")
}

pub fn write_output(
	args: &ArgMatches,
	buf: Vec<u8>,
	input: Option<PathBuf>,
	format: &str,
) -> Result<()> {
	if args.contains_id("Output directory") {
		let mut output_folder = match () {
			_ if let Some(path) = args.get_one::<String>("Output directory") => {
				let buf = PathBuf::from_str(path)?;
				if buf.is_absolute() {
					buf
				} else {
					let exec_dir = std::env::current_dir()?;
					exec_dir.join(buf)
				}
			},
			_ => input
				.context("No output path was specified, and the input was not a file")?
				.to_owned(),
		};

		match format {
			"Json" => {
				output_folder.set_extension("json");
				let mut file = OpenOptions::new()
					.write(true)
					.create(true)
					.open(output_folder)?;
				file.write_all(&buf)?;
			},
			"BlkText" => {
				output_folder.set_extension("blkx");
				let mut file = OpenOptions::new()
					.write(true)
					.create(true)
					.open(output_folder)?;
				file.write_all(&buf)?;
			},
			_ => {
				panic!("Unrecognized format: {format}")
			},
		}
		return Ok(());
	}

	if Some(&true) == args.get_one::<bool>("stdout") {
		io::stdout().write_all(&buf)?;
		return Ok(());
	}

	bail!("No output location passed. Pass an explicit output such as a file or stdout")
}
