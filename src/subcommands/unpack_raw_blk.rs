use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use clap::ArgMatches;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::{debug, info, warn};
use wt_blk::binary::{DecoderDictionary};
use wt_blk::binary::file::FileType;
use wt_blk::binary::nm_file::NameMap;
use wt_blk::binary::parser::parse_blk;
use wt_blk::binary::zstd::{BlkDecoder, decode_zstd};
use crate::error::CliError;
use crate::fs_util::{find_dict, read_recurse_folder};
use crate::task_queue::FileTask;

// This is the entry-point
pub fn unpack_raw_blk(args: &ArgMatches) -> Result<(), CliError> {
	info!("Mode: Unpacking raw BLK directory");
	let input_dir = args.get_one::<String>("Input directory").ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir).or(Err(CliError::InvalidPath))?;
	let input_read_dir = fs::read_dir(input_dir)?;

	let output_folder = match () {
		_ if let Some(path) = args.get_one::<String>("Output directory") =>  {
			let parent_folder = parsed_input_dir.parent().ok_or(CliError::InvalidPath)?;
			parent_folder.join(path)
		}
		_ if args.get_one::<bool>("Overwrite") == Some(&true) =>  {
			parsed_input_dir.clone()
		}
		_ => {
			let full_parent_folder = parsed_input_dir.parent().ok_or(CliError::InvalidPath)?;
			let parent_folder = full_parent_folder.file_name().unwrap().to_str().unwrap();
			full_parent_folder.join(parent_folder.to_owned() + "_unpacked")
		}
	};

	info!("Preparing files from folder into memory");
	let mut prepared_files = vec![];
	read_recurse_folder(&mut prepared_files, input_read_dir).unwrap();

	// The shared name map must always reside at the top level
	info!("Reading NM file");
	let nm = fs::read(format!("{}/nm", input_dir)).unwrap();

	// Dict files have hashed prefixes, so we find them via their file-name suffix and or magic bytes
	info!("Auto-detecting dict file");
	let (name, dict) = find_dict(input_dir).unwrap();
	info!("Found dict at {}", name);

	info!("Preparing shared indexes");
	let frame_decoder = DecoderDictionary::copy(&dict);
	let shared_nm = NameMap::from_encoded_file(&nm).unwrap();

	let rc_nm = Rc::new(shared_nm);
	let arced_fd = Arc::new(frame_decoder);
	info!("Parsing BLK into IR");

	let bar = Arc::new(ProgressBar::new(0));
	bar.set_style(
		ProgressStyle::with_template(" [{elapsed}/{eta}] [{wide_bar:.cyan/blue}] {percent}% {pos}/{len}").unwrap().progress_chars("#>-")
	);
	bar.set_length(prepared_files.len() as u64);

	let out = prepared_files.into_iter().map(|file| {
		let out = parse_file(file.1, arced_fd.clone(), rc_nm.clone());
		if out.is_none() {
			warn!("Failed to parse file {:?}", file.0)
		}
		bar.inc(1);
		if let Some(item) = out {
			Some((file.0, item))
		} else {
			None
		}
	}).filter_map(|x| x)
				  .collect::<Vec<_>>();
	bar.finish();


	info!("Writing parsed files");
	for file in out {
		let e = file.0.strip_prefix(parsed_input_dir.clone()).unwrap();
		let out = output_folder.join(e);
		fs::create_dir_all(out.clone().parent().unwrap()).unwrap();
		fs::write(out, file.1).unwrap();
		debug!("Successfully written {e:?}")
	}
	info!("All files are written");

	Ok(())
}

fn parse_file(mut file: Vec<u8>, fd: Arc<BlkDecoder>, shared_name_map: Rc<NameMap>) -> Option<String> {
	let mut offset = 0;
	let file_type = FileType::from_byte(file[0])?;
	if file_type.is_zstd() {
		file = decode_zstd(&file, fd.clone()).unwrap();
	} else {
		// uncompressed Slim and Fat files retain their initial magic bytes
		offset = 1;
	};


	let parsed = parse_blk(&file[offset..],  file_type.is_slim(), shared_name_map).ok()?;
	Some(serde_json::to_string_pretty(&parsed).ok()?)
}