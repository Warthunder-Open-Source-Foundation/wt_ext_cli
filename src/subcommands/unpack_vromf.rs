use std::{fs, path::PathBuf, str::FromStr, thread, thread::JoinHandle};

use anyhow::Context;
use clap::ArgMatches;
use tracing::info;
use wt_blk::{
	blk::{output_formatting_conf::FormattingConfiguration, BlkOutputFormat},
	vromf::unpacker::VromfUnpacker,
};

use crate::{
	context,
	error::{CliError, CliError::FileWithoutParent},
};

pub fn unpack_vromf(args: &ArgMatches) -> Result<(), anyhow::Error> {
	info!("Mode: Unpacking vromf");
	let input_dir = args
		.get_one::<String>("Input file/directory")
		.ok_or(CliError::RequiredFlagMissing)?;
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

	let mode = match args.get_one::<String>("format").map(|e| e.as_str()) {
		Some("Json") => Some(BlkOutputFormat::Json(FormattingConfiguration::GSZABI_REPO)),
		Some("BlkText") => Some(BlkOutputFormat::BlkText),
		Some("Raw") => None,
		_ => {
			panic!(
				"Unrecognized output format: {:?}",
				args.get_one::<String>("format")
			)
		},
	};

	if parsed_input_dir.is_dir() {
		let mut threads: Vec<Box<JoinHandle<Result<(), anyhow::Error>>>> = vec![];
		let inner = fs::read_dir(&parsed_input_dir)?;
		for file in inner {
			if let Ok(file) = file {
				if file
					.file_name()
					.to_str()
					.unwrap_or("")
					.ends_with("vromfs.bin")
				{
					let parsed_input_dir = parsed_input_dir.clone();
					let output_folder = output_folder.clone();
					threads.push(Box::new(thread::spawn(move || {
						let read = fs::read(file.path()).with_context(context!(format!(
							"Failed to read vromf {:?}",
							file.path()
						)))?;
						let parent_input_dir = parsed_input_dir
							.parent()
							.ok_or(FileWithoutParent)?
							.to_path_buf();
						let normalized = parent_input_dir.canonicalize()?;
						parse_and_write_one_vromf(
							&file.file_name().into_string().unwrap(),
							read,
							normalized,
							output_folder,
							mode,
						)?;
						Ok(())
					})))
				}
			}
		}
		for thread in threads {
			thread.join().expect("Thread join error")?
		}
	} else {
		let read = fs::read(&parsed_input_dir)?;
		let parent_input_dir = parsed_input_dir
			.parent()
			.ok_or(FileWithoutParent)?
			.to_path_buf();
		let normalized = parent_input_dir.canonicalize()?;
		parse_and_write_one_vromf(input_dir, read, normalized, output_folder, mode)?;
	}

	Ok(())
}

fn parse_and_write_one_vromf(
	file_name: &str,
	read: Vec<u8>,
	_input_dir: PathBuf,
	_output_dir: PathBuf,
	format: Option<BlkOutputFormat>,
) -> Result<(), anyhow::Error> {
	let parser = VromfUnpacker::from_file((PathBuf::from_str(file_name)?, read))?;
	let _files = parser.unpack_all(format)?;
	// parse_and_write_blk(
	// 	vromf_inner,
	// 	format,
	// 	nm.1,
	// 	dict.1,
	// 	input_dir,
	// 	output_dir,
	// 	strip_and_add_prefix,
	// )
	Ok(())
}
