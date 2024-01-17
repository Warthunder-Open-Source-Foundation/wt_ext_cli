use std::{fs, path::PathBuf, str::FromStr, thread, thread::JoinHandle};
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::ops::ControlFlow;
use std::sync::Arc;

use clap::ArgMatches;
use color_eyre::eyre::{Context, ContextCompat, Result};
use color_eyre::Help;
#[cfg(feature = "avif2dds")]
use image::ImageFormat;
use tracing::info;
use wt_blk::vromf::{BlkOutputFormat, VromfUnpacker};
use zip::CompressionMethod;
use zip::write::FileOptions;

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

	let zip = *args.get_one::<bool>("zip").context("Invalid argument: zip")?;

	let should_override = *args.get_one::<bool>("override").context("Invalid argument: override")?;

	let avif2dds = *args.get_one::<bool>("avif2dds").context("Invalid argument: avif2dds")?;

	let blk_extension = args.get_one::<String>("blk_extension").map(|e| Arc::new(e.to_owned()));

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
					let blk_extension = blk_extension.clone();
					threads.push(Box::new(thread::spawn(move || {
						let read = fs::read(file.path()).with_context(context!(format!(
							"Failed to read vromf {:?}",
							file.path()
						)))?;
						parse_and_write_one_vromf(file.path(), read, output_folder, mode, crlf, should_override, avif2dds, zip, blk_extension)
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
		parse_and_write_one_vromf(parsed_input_dir, read, output_folder, mode, crlf, should_override, avif2dds, zip, blk_extension)?;
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
	#[allow(unused)] // Conditionally depending on target
	avif2dds: bool,
	zip: bool,
	blk_extension: Option<Arc<String>>,
) -> Result<()> {
	let parser = VromfUnpacker::from_file((file_path.clone(), read))?;

	let mut vromf_name = PathBuf::from(file_path.file_name().ok_or(CliError::InvalidPath)?);
	let mut old_extension = vromf_name
		.extension()
		.ok_or(CliError::InvalidPath)?
		.to_os_string();
	old_extension.push("_u");
	vromf_name.set_extension(old_extension);

	let writer = |file: &mut (PathBuf, Vec<u8>)| {
		{
			// The version file in some vromfs is prefixed with /, which is incorrect as this causes
			// all relative paths to resolve to /
			if file.0.starts_with("/") {
				file.0 = file.0.strip_prefix("/")?.to_path_buf();
			}
			if crlf {
				if file.0.extension() == Some(&OsStr::new("blk")) {
					let mut new = Vec::with_capacity(file.1.len() + 1024 * 4);
					for byte in &file.1 {
						if *byte == b'\n' {
							new.push(b'\r');
						}
						new.push(*byte);
					}
					file.1 = new;
				}
			}
			#[cfg(feature = "avif2dds")]
			if avif2dds {
				if file.0.extension() == Some(&OsStr::new("avif")) {
					let image = image::load_from_memory_with_format(&file.1, ImageFormat::Avif);
					match image {
						Ok(image) => {
							file.1.clear();
							image.write_to(&mut std::io::Cursor::new(&mut file.1), ImageFormat::Dds)?;
							file.0.set_extension("dds");
						}
						Err(e) => {
							tracing::warn!("{} was unable to convert to PNG because of: {e}", file.0.to_string_lossy());
						}
					}
				}
			}
			let rel_file_path = vromf_name.clone().join(&file.0);
			let mut joined_final_path = output_dir.join(&rel_file_path);

			if let Some(extension) = blk_extension.clone() {
				if joined_final_path.extension() == Some(OsStr::new("blk")) {
					joined_final_path.set_extension(extension.as_str());
				}
			}

			fs::create_dir_all(joined_final_path.parent().ok_or(CliError::InvalidPath)?)?;
			let handle = OpenOptions::new().write(true).create_new(true).open(&joined_final_path)?;
			Ok(BufWriter::with_capacity(4096, handle))
		}
	};

	parser.unpack_all_with_writer(format, should_override, writer)?;

	let (sender, receiver) = std::sync::mpsc::channel();
	let handle = if zip {
		let output_dir = output_dir.clone();
		let handle = thread::spawn(move || {
			let mut file = File::create(output_dir).unwrap();

			let mut writer = zip::ZipWriter::new(&mut file);

			loop {
				let con: ControlFlow<(), (Vec<u8>, PathBuf)> = receiver.recv().unwrap();

				match con {
					ControlFlow::Continue((buffer, path)) => {
						writer.start_file(path.to_string_lossy(), FileOptions::default().compression_method(CompressionMethod::Deflated)).unwrap();
						writer.write_all(&buffer).unwrap();
					}
					ControlFlow::Break(_) => {
						break;
					}
				}
			}
		});
		Some(handle)
	} else {
		None
	};

	if let Some(thread) = handle {
		sender.send(ControlFlow::Break(()))?;
		thread.join().unwrap();
	}

	Ok(())
}
