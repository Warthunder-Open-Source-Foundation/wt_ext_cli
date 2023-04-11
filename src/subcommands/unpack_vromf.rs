use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use clap::ArgMatches;
use tracing::info;
use wt_blk::vromf::decode_vromf;
use crate::error::CliError;
use crate::subcommands::unpack_raw_blk::parse_and_write_blk;

pub fn unpack_vromf(args: &ArgMatches) -> Result<(), CliError> {
	info!("Mode: Unpacking vromf");
	let input_dir = args.get_one::<String>("Input file/directory").ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir).or(Err(CliError::InvalidPath))?;

	let output_folder = match () {
		_ if let Some(path) = args.get_one::<String>("Output directory") => {
			let parent_folder = parsed_input_dir.parent().ok_or(CliError::InvalidPath)?;
			parent_folder.join(path)
		}
		_ => {
			let full_parent_folder = parsed_input_dir.parent().ok_or(CliError::InvalidPath)?;
			let parent_folder = full_parent_folder.file_name().unwrap().to_str().unwrap();
			full_parent_folder.join(parent_folder.to_owned() + "_u")
		}
	};

	if parsed_input_dir.is_dir() {
		let inner = fs::read_dir(&parsed_input_dir)?;
	} else {
		let read = fs::read(&parsed_input_dir)?;
		parse_and_write_one_vromf(&read, parsed_input_dir, output_folder);
	}

	Ok(())
}

fn parse_and_write_one_vromf(read: &[u8], input_dir: PathBuf, output_dir: PathBuf) {
	let mut vromf_inner = decode_vromf(read).into_iter().map(|x|(PathBuf::from_str(&x.0).unwrap(), x.1)).collect::<Vec<_>>();

	let nm = vromf_inner.iter().find(|x|x.0 == PathBuf::from_str("nm").unwrap()).unwrap().to_owned();
	let dict = vromf_inner.iter().find(|x|x.0.extension() == Some(OsStr::new("dict"))).unwrap().to_owned();

	parse_and_write_blk(vromf_inner,nm.1, dict.1, input_dir, output_dir).unwrap();
}