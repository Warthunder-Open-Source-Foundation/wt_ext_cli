use std::{fs, path::PathBuf, str::FromStr, thread, thread::JoinHandle};
use std::ffi::OsStr;
#[cfg(feature = "avif2dds")]
use std::io::{Cursor};

use clap::ArgMatches;
use color_eyre::eyre::{Context, ContextCompat, Result};
use color_eyre::Help;
#[cfg(feature = "avif2dds")]
use image::{ImageFormat, ImageResult};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::{info, warn};
use wt_blk::vromf::{BlkOutputFormat, VromfUnpacker};

use crate::{context, error::CliError};
pub fn unpack_vromf(args: &ArgMatches) -> Result<()> {
	info!("Mode: Unpacking vromf");
	let input_dir = args
		.get_one::<String>("Input file/directory")
		.ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir).or(Err(CliError::InvalidPath))?;

	let mode = match args.get_one::<String>("format").map(|e| e.as_str()) {
		Some("Json") => Some(BlkOutputFormat::Json),
		Some("BlkText") => Some(BlkOutputFormat::BlkText),
		Some("Raw") => None,
		_ => {
			panic!(
				"Unrecognized output format: {:?}",
				args.get_one::<String>("format")
			)
		}
	};

	let crlf = *args.get_one::<bool>("crlf").context("Invalid argument: crlf")?;

	let should_override = *args.get_one::<bool>("override").context("Invalid argument: override")?;

	let avif2dds = *args.get_one::<bool>("avif2dds").context("Invalid argument: avif2dds")?;

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

		let mut threads: Vec<Box<JoinHandle<Result<()>>>> = vec![];
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
					threads.push(Box::new(thread::spawn(move || {
						let read = fs::read(file.path()).with_context(context!(format!(
							"Failed to read vromf {:?}",
							file.path()
						)))?;
						parse_and_write_one_vromf(file.path(), read, output_folder, mode, crlf, should_override, avif2dds)
							.suggestion(format!("Error filename: {}", file.file_name().to_string_lossy()))?;
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
		parse_and_write_one_vromf(parsed_input_dir, read, output_folder, mode, crlf, should_override, avif2dds)?;
	}

	Ok(())
}

fn parse_and_write_one_vromf(
	file_path: PathBuf,
	read: Vec<u8>,
	output_dir: PathBuf,
	format: Option<BlkOutputFormat>,
	crlf: bool,
	should_override: bool,
	avif2dds: bool,
) -> Result<()> {
	let parser = VromfUnpacker::from_file((file_path.clone(), read))?;
	let files = parser.unpack_all(format, should_override)?;

	let mut vromf_name = PathBuf::from(file_path.file_name().ok_or(CliError::InvalidPath)?);
	let mut old_extension = vromf_name
		.extension()
		.ok_or(CliError::InvalidPath)?
		.to_os_string();
	old_extension.push("_u");
	vromf_name.set_extension(old_extension);


	files
		.into_par_iter()
		.map(|mut file| {
			// The version file in some vromfs is prefixed with /, which is incorrect as this causes
			// all relative paths to resolve to /
			if file.0.starts_with("/") {
				file.0 = file.0.strip_prefix("/")?.to_path_buf();
			}
			if crlf {
				if file.0.extension() == Some(&OsStr::new("blk"))  {
					let mut new = Vec::with_capacity(file.1.len() + 1024 * 4);
					for byte in file.1 {
						if byte == b'\n' {
							new.push(b'\r');
						}
						new.push(byte);
					}
					file.1 = new;
				}
			}
			#[cfg(feature = "avif2dds")]
			if avif2dds {
				if file.0.extension() == Some(&OsStr::new("avif"))  {
					let image = image::load_from_memory_with_format(&file.1, ImageFormat::Avif);
					match image {
						Ok(image) => {
							file.1.clear();
							image.write_to(&mut Cursor::new(&mut file.1), ImageFormat::Dds)?;
							file.0.set_extension("dds");
						}
						Err(e) => {
							warn!("{} was unable to convert to PNG because of: {e}", file.0.to_string_lossy());
						}
					}
				}
			}
			let rel_file_path = vromf_name.clone().join(&file.0);
			let joined_final_path = output_dir.join(&rel_file_path);
			fs::create_dir_all(joined_final_path.parent().ok_or(CliError::InvalidPath)?)?;
			fs::write(&joined_final_path, file.1)?;
			Ok(())
		})
		.collect::<Result<()>>()?;

	Ok(())
}
