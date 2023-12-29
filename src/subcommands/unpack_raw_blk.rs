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

	let parsed = blk::unpack_blk(read, None, None)?;

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

	fs::write(output_folder,  serde_json::to_string_pretty(&parsed.as_serde_json(false).1)?)?;

	Ok(())
}

// pub fn unpack_raw_blk(args: &ArgMatches) -> Result<()> {
// info!("Mode: Unpacking raw BLK directory");
// let input_dir = args
// .get_one::<String>("Input directory")
// .ok_or(CliError::RequiredFlagMissing)?;
// let parsed_input_dir = PathBuf::from_str(&input_dir)
// .or(Err(CliError::InvalidPath))
// .context(format!(
// "The provided input directory {} is not valid",
// input_dir
// ))?;
// let input_read_dir = fs::read_dir(input_dir)?;
//
// let output_folder = match () {
// _ if let Some(path) = args.get_one::<String>("Output directory") => {
// let parent_folder = parsed_input_dir.parent().ok_or(CliError::InvalidPath)?;
// parent_folder.join(path)
// }
// _ if args.get_count("Overwrite") >= 1 => { TODO: Fix overwrite
// 	parsed_input_dir.clone()
// }
// _ => {
// let full_parent_folder = parsed_input_dir.parent().ok_or(CliError::InvalidPath)?;
// let parent_folder = full_parent_folder.file_name().unwrap().to_str().unwrap();
// full_parent_folder.join(parent_folder.to_owned() + "_u")
// }
// };
//
// info!("Preparing files from folder into memory");
// let mut prepared_files = vec![];
// read_recurse_folder(&mut prepared_files, input_read_dir).unwrap();
//
// The shared name map must always reside at the top level
// info!("Reading NM file");
// let nm = fs::read(format!("{}/nm", input_dir)).unwrap();
//
// Dict files have hashed prefixes, so we find them via their file-name suffix and or magic bytes
// info!("Auto-detecting dict file");
// let (name, dict) = find_dict(input_dir).unwrap();
// info!("Found dict at {}", name);
//
// parse_and_write_blk(
// prepared_files,
// OutFormat::Json,  // TODO: Replace default
// nm,
// dict,
// parsed_input_dir,
// output_folder,
// strip_and_add_prefix,
// )?;
//
// Ok(())
// }
//
// pub fn parse_and_write_blk(
// prepared_files: Vec<(PathBuf, Vec<u8>)>,
// format: OutFormat,
// nm: Vec<u8>,
// dict: Vec<u8>,
// input_dir: PathBuf,
// output_dir: PathBuf,
// output_file_path_builder: fn(PathBuf, PathBuf, PathBuf) -> Result<PathBuf, Error>, /* This fn should take care of turning a (maybe) relative path into a writable absolute/accessible path */
// ) -> Result<()> {
// info!("Preparing shared indexes");
// let frame_decoder = DecoderDictionary::copy(&dict);
// let shared_nm = NameMap::from_encoded_file(&nm).unwrap();
//
// let rc_nm = Arc::new(shared_nm);
// let arced_fd = Arc::new(frame_decoder);
// info!("Parsing BLK into IR");
//
// let bar = Arc::new(ProgressBar::new(0));
// bar.set_style(
// ProgressStyle::with_template(
// " [{elapsed}/{eta}] [{wide_bar:.cyan/blue}] {percent}% {pos}/{len}",
// )
// .unwrap()
// .progress_chars("#>-"),
// );
// bar.set_length(prepared_files.len() as u64);
//
// let out = prepared_files
// .into_iter()
// .map(|file| {
// Parse BLK files, copy the rest as-is
// let out = if file.0.extension() == Some(OsStr::new("blk"))
// && FileType::from_byte(file.1[0]).is_some()
// {
// parse_file(file.1, arced_fd.clone(), rc_nm.clone(), format)
// } else {
// Some(file.1)
// };
//
// if out.is_none() {
// warn!("Failed to parse file {:?}", file.0)
// }
// bar.inc(1);
// if let Some(item) = out {
// Some((file.0, item))
// } else {
// None
// }
// })
// .filter_map(|x| x)
// .collect::<Vec<_>>();
// bar.finish();
//
// info!("Writing parsed files");
// for file in out {
// let file_out_dir = output_file_path_builder(file.0, input_dir.clone(), output_dir.clone())?;
// fs::create_dir_all(file_out_dir.clone().parent().unwrap()).unwrap();
//
// fs::write(&file_out_dir, file.1)
// .context(format!("Failed to write output file to {:?}", file_out_dir))?;
// }
// info!("All files are written");
//
// Ok(())
// }
//
// fn parse_file(
// mut file: Vec<u8>,
// fd: Arc<BlkDecoder>,
// shared_name_map: Arc<NameMap>,
// format: OutFormat,
// ) -> Option<Vec<u8>> {
// let mut offset = 0;
// let file_type = FileType::from_byte(file[0])?;
// if file_type.is_zstd() {
// file = decode_zstd(&file, fd.clone()).unwrap();
// } else {
// uncompressed Slim and Fat files retain their initial magic bytes
// offset = 1;
// };
//
// let parsed = parse_blk(&file[offset..], file_type.is_slim(), shared_name_map).ok()?;
// return match format {
// OutFormat::BlkText => {
// Some(parsed.as_blk_text().into_bytes())
// }
// OutFormat::Raw => {
// Some(file)
// }
// OutFormat::Json => {
// Some(parsed.as_ref_json(FormattingConfiguration::GSZABI_REPO).into_bytes())
// }
// }
// }
//
// fn strip_and_add_prefix(
// input: PathBuf,
// input_dir: PathBuf,
// output_dir: PathBuf,
// ) -> Result<PathBuf, anyhow::Error> {
// let e = input
// .strip_prefix(input_dir.clone())
// .with_context(context!(format!(
// "Failed to strip prefix {:?} from base {:?}",
// input_dir.clone(),
// input
// )))?;
//
// Ok(output_dir.join(e))
// }
