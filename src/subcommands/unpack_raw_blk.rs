use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use clap::ArgMatches;
use color_eyre::eyre::{bail, Result};
use wt_blk::blk;
use wt_blk::blk::file::FileType;
use crate::error::CliError;

// This is the entry-point
pub fn unpack_raw_blk(args: &ArgMatches) -> Result<()> {
	let input = args
		.get_one::<String>("Input directory")
		.ok_or(CliError::RequiredFlagMissing)?;
	let input = Path::new(input);
	let read = fs::read(input)?;

	let zstd_dict = None;
	let nm = None;

	match FileType::from_byte(read[0])? {
		FileType::BBF => {}
		FileType::FAT => {}
		FileType::FAT_ZSTD => {}
		FileType::SLIM => {
			bail!("External name-map is not implemented yet");
		}
		FileType::SLIM_ZSTD => {
			bail!("External name-map is not implemented yet");
		}
		FileType::SLIM_ZST_DICT => {
			bail!("ZSTD dictionary is not implemented yet");
		}
	}

	let mut parsed = blk::unpack_blk(read, zstd_dict, nm)?;

	let mut output_folder = match () {
		_ if let Some(path) = args.get_one::<String>("Output directory") => {
			let buf = PathBuf::from_str(path)?;
			if buf.is_absolute() {
				buf
			} else {
				let exec_dir = std::env::current_dir()?;
				exec_dir.join(buf)
			}
		}
		_ => {
			input.to_owned()
		}
	};

	output_folder.set_extension("json");

	fs::write(output_folder,  serde_json::to_string_pretty(&mut parsed.as_serde_json(false).1)?)?;

	Ok(())
}