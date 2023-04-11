use std::ffi::OsStr;
use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread::JoinHandle;
use anyhow::Context;
use clap::ArgMatches;
use tracing::info;
use wt_blk::vromf::decode_vromf;
use crate::context;
use crate::error::CliError;
use crate::error::CliError::{CriticalFileMissing, FileWithoutParent};
use crate::subcommands::unpack_raw_blk::parse_and_write_blk;

pub fn unpack_vromf(args: &ArgMatches) -> Result<(), anyhow::Error> {
	info!("Mode: Unpacking vromf");
	let input_dir = args.get_one::<String>("Input file/directory").ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir).or(Err(CliError::InvalidPath))?;

	let output_folder = match () {
		_ if let Some(path) = args.get_one::<String>("Output directory") => {
			let parent_folder = parsed_input_dir.parent().ok_or(CliError::InvalidPath)?;
			parent_folder.join(path)
		}
		_ => {
			PathBuf::from_str(&format!("{}_u", input_dir))?
		}
	};

	if parsed_input_dir.is_dir() {
		let mut threads: Vec<Box<JoinHandle<Result<(), anyhow::Error>>>> = vec![];
		let inner = fs::read_dir(&parsed_input_dir)?;
		for file in inner {
			if let Ok(file) = file {
				if file.file_name().to_str().unwrap_or("").ends_with("vromfs.bin") {
					let parsed_input_dir = parsed_input_dir.clone();
					let output_folder = output_folder.clone();
					threads.push(Box::new(
						thread::spawn(move ||{
							let read = fs::read(file.path()).with_context(context!(format!("Failed to read vromf {:?}", file.path())))?;
							let parent_input_dir = parsed_input_dir.parent().ok_or(FileWithoutParent)?.to_path_buf();
							let normalized = parent_input_dir.canonicalize()?;
							parse_and_write_one_vromf(&file.file_name().into_string().unwrap(), &read, normalized, output_folder)?;
							Ok(())
						})
					))
				}
			}
		}
		for thread in threads {
			thread.join().expect("Thread join error")?
		}
	} else {
		let read = fs::read(&parsed_input_dir)?;
		let parent_input_dir = parsed_input_dir.parent().ok_or(FileWithoutParent)?.to_path_buf();
		let normalized = parent_input_dir.canonicalize()?;
		parse_and_write_one_vromf(input_dir, &read, normalized, output_folder)?;
	}

	Ok(())
}

fn parse_and_write_one_vromf(file_name: &str, read: &[u8], input_dir: PathBuf, output_dir: PathBuf) -> Result<(), anyhow::Error> {
	let mut vromf_inner = decode_vromf(read).into_iter().map(|x|(PathBuf::from_str(&x.0).unwrap(), x.1)).collect::<Vec<_>>();

	let nm = vromf_inner.iter()
		.find(|x|
			x.0 == PathBuf::from_str("nm").expect("infallible")
		)
		.ok_or(CriticalFileMissing)
		.with_context(context!(format!("Failed to find Name Map (nm) in vromf {}", file_name)))?
		.to_owned();

	let dict = vromf_inner.iter()
		.find(|x|
			x.0.extension() == Some(OsStr::new("dict"))
		).with_context(context!(format!("Failed to find ZST dictionary in vromf {}", file_name)))?
		.to_owned();

	parse_and_write_blk(vromf_inner,nm.1, dict.1, input_dir, output_dir, strip_and_add_prefix)
}

fn strip_and_add_prefix(input: PathBuf, _: PathBuf, output_dir: PathBuf) -> Result<PathBuf, anyhow::Error> {
	Ok(output_dir.join(input))
}