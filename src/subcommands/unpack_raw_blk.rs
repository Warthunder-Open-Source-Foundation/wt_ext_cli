use std::{
	fs,
	fs::OpenOptions,
	io::Write,
	path::{Path, PathBuf},
	str::FromStr,
};

use clap::ArgMatches;
use color_eyre::eyre::{bail, ContextCompat, Result};
use wt_blk::{blk, blk::file::FileType};

use crate::error::CliError;

// This is the entry-point
pub fn unpack_raw_blk(args: &ArgMatches) -> Result<()> {
	let input = args
		.get_one::<String>("Input directory")
		.ok_or(CliError::RequiredFlagMissing)?;

	let format = args
		.get_one::<String>("format")
		.context("Invalid format specified or missing")?;

	let input = Path::new(input);
	if input.is_dir() {
		bail!("Directories as input are not implemented yet");
	}

	let mut read = fs::read(input)?;

	let zstd_dict = None;
	let nm = None;

	match FileType::from_byte(read[0])? {
		FileType::BBF => {},
		FileType::FAT => {},
		FileType::FAT_ZSTD => {},
		FileType::SLIM => {
			bail!("External name-map is not implemented yet");
		},
		FileType::SLIM_ZSTD => {
			bail!("External name-map is not implemented yet");
		},
		FileType::SLIM_ZST_DICT => {
			bail!("ZSTD dictionary is not implemented yet");
		},
	}

	let mut parsed = blk::unpack_blk(&mut read, zstd_dict, nm)?;

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
		_ => input.to_owned(),
	};

	//output_folder.push(input.file_name().unwrap().to_string_lossy().to_string());
	match format.as_str() {
		"Json" => {
			output_folder.set_extension("json");
			let mut file = OpenOptions::new()
				.write(true)
				.create(true)
				.open(output_folder)?;

			parsed.merge_fields();
			file.write_all(&parsed.as_serde_json()?)?;
		},
		"BlkText" => {
			output_folder.set_extension("blkx");
			let mut file = OpenOptions::new()
				.write(true)
				.create(true)
				.open(output_folder)?;
			file.write_all(parsed.as_blk_text()?.as_bytes())?;
		},
		_ => {
			panic!("Unrecognized format: {format}")
		},
	}

	Ok(())
}
