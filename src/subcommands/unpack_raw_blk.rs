use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use clap::ArgMatches;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;
use wt_blk::binary::{DecoderDictionary, parse_file};
use wt_blk::binary::nm_file::NameMap;
use crate::error::CliError;
use crate::fs_util::{find_dict, read_recurse_folder};
use crate::task_queue::FileTask;

// This is the entry-point
pub fn unpack_raw_blk(args: &ArgMatches) -> Result<(), CliError> {
	// This should be infallible
	info!("Mode: Unpacking raw BLK directory");
	let input_dir = args.get_one::<String>("Input directory").ok_or(CliError::RequiredFlagMissing)?;
	let input_read_dir = fs::read_dir(input_dir)?;

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
		bar.inc(1);
		out
	}).filter_map(|x| x)
				  .collect::<Vec<_>>();
	bar.finish();

	Ok(())
}