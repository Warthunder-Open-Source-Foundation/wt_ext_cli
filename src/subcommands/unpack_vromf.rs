use std::{
	ffi::OsStr,
	fs,
	fs::{File, OpenOptions},
	io::{BufWriter, Write},
	mem::take,
	ops::ControlFlow,
	path::PathBuf,
	str::FromStr,
	sync::Arc,
	thread,
	thread::JoinHandle,
};

use clap::{parser::ValueSource, ArgMatches};
use color_eyre::{
	eyre::{bail, ContextCompat, Result},
	Help,
};
use log::{error, info};
use wt_blk::{
	blk::util::maybe_blk,
	vromf::{BlkOutputFormat, File as BlkFile, VromfUnpacker},
};
use zip::{write::SimpleFileOptions, CompressionMethod};

use crate::{
	arced,
	error::CliError,
	image_conversion::{Converter, ImageConverter},
	util::CrlfWriter,
};

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
		},
	};

	let crlf = *args
		.get_one::<bool>("crlf")
		.context("Invalid argument: crlf")?;

	let zip = *args
		.get_one::<bool>("zip")
		.context("Invalid argument: zip")?;

	let skip_integrity_check = *args
		.get_one::<bool>("skip_integrity_check")
		.context("Invalid argument: skip_integrity_check")?;
	let check_integrity = !skip_integrity_check;

	let should_override = *args
		.get_one::<bool>("override")
		.context("Invalid argument: override")?;

	let avif2png = args
		.get_one::<String>("avif2png")
		.context("Invalid argument: avif2png")?;

	let blk_extension = args
		.get_one::<String>("blk_extension")
		.map(|e| Arc::new(e.to_owned()));

	let folder = args.get_one::<String>("folder").map(ToOwned::to_owned);

	let mut ffmpeg = ImageConverter::new_with_converter(Converter::new_from_arg(&avif2png)?);
	let mut avif2png = false;

	if args
		.value_source("avif2png")
		.context("infallible")?
		.ne(&ValueSource::DefaultValue)
	{
		ffmpeg.validate()?;
		avif2png = true;
	}
	let ffmpeg = Arc::new(ffmpeg);

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
			},
			_ => parsed_input_dir.clone(),
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
					arced!(output_folder, blk_extension, ffmpeg);
					let thread_builder =
						thread::Builder::new().name(file.file_name().to_string_lossy().to_string());
					let folder = folder.clone();

					threads.push(Box::new(thread_builder.spawn(move || {
						parse_and_write_one_vromf(
							BlkFile::new(file.path())?,
							output_folder,
							mode,
							crlf,
							should_override,
							avif2png,
							zip,
							blk_extension,
							ffmpeg,
							check_integrity,
							folder,
						)
						.suggestion(format!(
							"Error filename: {}",
							file.file_name().to_string_lossy()
						))?;
						Ok(())
					})?))
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
			},
			_ => parsed_input_dir
				.clone()
				.parent()
				.ok_or(CliError::InvalidPath)?
				.to_owned(),
		};
		parse_and_write_one_vromf(
			BlkFile::new(parsed_input_dir)?,
			output_folder,
			mode,
			crlf,
			should_override,
			avif2png,
			zip,
			blk_extension,
			ffmpeg,
			check_integrity,
			folder,
		)?;
	}

	Ok(())
}

fn parse_and_write_one_vromf(
	file: BlkFile,
	output_dir: PathBuf,
	format: Option<BlkOutputFormat>,
	crlf: bool,
	should_override: bool,
	#[allow(unused)] // Conditionally depending on target
	avif2png: bool,
	zip: bool,
	blk_extension: Option<Arc<String>>,
	ffmpeg: Arc<ImageConverter>,
	check_integrity: bool,
	subdir: Option<String>,
) -> Result<()> {
	if let Some(meta) = file.meta() {
		match meta.len() {
			0 => {
				bail!("Vromf is zero bytes long {:?}", file.path())
			},
			len @ 0..=1000 => {
				error!("Vromf is very small ({len} bytes) {:?}", file.path())
			},
			_ => {},
		}
	}

	let parser = VromfUnpacker::from_file(&file, check_integrity)?;

	let mut vromf_name = PathBuf::from(file.path().file_name().ok_or(CliError::InvalidPath)?);
	let mut old_extension = vromf_name
		.extension()
		.ok_or(CliError::InvalidPath)?
		.to_os_string();
	old_extension.push("_u");
	vromf_name.set_extension(old_extension);

	let writer = |file: &mut BlkFile| {
		{
			// The version file in some vromfs is prefixed with /, which is incorrect as this causes
			// all relative paths to resolve to /
			if file.path().starts_with("/") {
				*file.path_mut() = file.path().strip_prefix("/")?.to_path_buf();
			}

			let rel_file_path = vromf_name.clone().join(file.path());
			let mut joined_final_path = output_dir.join(&rel_file_path);

			let is_blk = maybe_blk(file);

			if let Some(extension) = blk_extension.clone() {
				if is_blk {
					joined_final_path.set_extension(extension.as_str());
				}
			}
			fs::create_dir_all(joined_final_path.parent().ok_or(CliError::InvalidPath)?)?;

			if avif2png {
				if file.path().extension() == Some(&OsStr::new("avif")) {
					// Convert image
					joined_final_path.set_extension("png");
					ffmpeg.convert_and_write(
						take(&mut file.buf_mut()),
						joined_final_path
							.to_str()
							.context("Final path is not a valid str")?,
					)?;
					return Ok(CrlfWriter::Null);
				}
			}
			let handle = OpenOptions::new()
				.write(true)
				.create(true)
				.open(&joined_final_path)?;
			let buf_size = 2_usize.pow(20); // Megabyte
			if crlf && is_blk {
				Ok(CrlfWriter::Enabled(BufWriter::with_capacity(
					buf_size, handle,
				)))
			} else {
				Ok(CrlfWriter::Disabled(BufWriter::with_capacity(
					buf_size, handle,
				)))
			}
		}
	};

	parser.unpack_all_with_writer(format, should_override, writer, true)?;

	let (sender, receiver) = std::sync::mpsc::channel();
	let handle = if zip {
		let output_dir = output_dir.clone();

		let thread_builder = thread::Builder::new().name("zip_writer".to_owned());
		let handle = thread_builder.spawn(move || {
			let file = File::create(output_dir).unwrap();
			let mut file = BufWriter::new(file);

			let mut writer = zip::ZipWriter::new(&mut file);
			let options =
				SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

			loop {
				let con: ControlFlow<(), (Vec<u8>, PathBuf)> = receiver.recv().unwrap();

				match con {
					ControlFlow::Continue((buffer, path)) => {
						writer.start_file(path.to_string_lossy(), options).unwrap();
						writer.write_all(&buffer).unwrap();
					},
					ControlFlow::Break(_) => {
						break;
					},
				}
			}
		});
		Some(handle)
	} else {
		None
	};

	if let Some(thread) = handle {
		sender.send(ControlFlow::Break(()))?;
		thread?.join().unwrap();
	}

	Ok(())
}
