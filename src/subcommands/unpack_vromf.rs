use std::{fs, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::ArgMatches;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use tokio::{join, try_join};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tracing::info;
use wt_blk::{
	blk::{output_formatting_conf::FormattingConfiguration, BlkOutputFormat},
	vromf::unpacker::VromfUnpacker,
};

use crate::{context, error::CliError};

pub fn unpack_vromf(args: &ArgMatches) -> Result<(), anyhow::Error> {
	info!("Mode: Unpacking vromf");
	let input_dir = args
		.get_one::<String>("Input file/directory")
		.ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir).or(Err(CliError::InvalidPath))?;

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
		let output_folder = match () {
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
				parsed_input_dir.clone()
			}
		};

		let mut threads: Vec<Box<std::thread::JoinHandle<Result<(), anyhow::Error>>>> = vec![];
		let inner = fs::read_dir(&parsed_input_dir)?;
		for file in inner {
			if let Ok(file) = file {
				if file
					.file_name()
					.to_str()
					.unwrap_or("")
					.ends_with("vromfs.bin")
				{
					let output_folder = output_folder.clone();
					threads.push(Box::new(std::thread::spawn(move || {
						let read = fs::read(file.path()).with_context(context!(format!(
							"Failed to read vromf {:?}",
							file.path()
						)))?;
						parse_and_write_one_vromf(file.path(), read, output_folder, mode)?;
						Ok(())
					})))
				}
			}
		}
		for thread in threads {
			thread.join().expect("Thread join error")?
		}
	} else {
		let output_folder = match () {
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
				parsed_input_dir.clone().parent().ok_or(CliError::InvalidPath)?.to_owned()
			}
		};
		let read = fs::read(&parsed_input_dir)?;
		parse_and_write_one_vromf(parsed_input_dir, read, output_folder, mode)?;
	}

	Ok(())
}

fn parse_and_write_one_vromf(
	file_path: PathBuf,
	read: Vec<u8>,
	output_dir: PathBuf,
	format: Option<BlkOutputFormat>,
) -> Result<(), anyhow::Error> {
	let parser = VromfUnpacker::from_file((file_path.clone(), read))?;
	let files = parser.unpack_all(format)?;

	let mut vromf_name = PathBuf::from(file_path.file_name().ok_or(CliError::InvalidPath)?);
	let mut old_extension = vromf_name
		.extension()
		.ok_or(CliError::InvalidPath)?
		.to_os_string();
	old_extension.push("_u");
	vromf_name.set_extension(old_extension);

	let rt = Runtime::new()?;
	let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = vec![];

	for mut file in files {
		let vromf_name = vromf_name.clone();
		let output_dir = output_dir.clone();
		let handle = rt.spawn(async move {
			// The version file in some vromfs is prefixed with /, which is incorrect as this causes
			// all relative paths to resolve to /
			if file.0.starts_with("/") {
				file.0 = file.0.strip_prefix("/")?.to_path_buf();
			}
			let rel_file_path = vromf_name.join(&file.0);
			let joined_final_path = output_dir.join(&rel_file_path);
			tokio::fs::create_dir_all(joined_final_path.parent().ok_or(CliError::InvalidPath)?).await?;
			tokio::fs::write(&joined_final_path, file.1).await?;
			Ok(())
		});
		handles.push(handle);
	}

	let mut results = vec![];
	rt.block_on(async {
		for handle in handles {
			results.push(handle.await);
		}
	});
	for result in results {
		result??;
	}

	Ok(())
}
