mod args;
mod fs_util;

use std::borrow::Cow;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::{fs, thread};
use std::fs::ReadDir;
use std::io::stdout;
use std::rc::Rc;
use std::thread::{sleep, spawn};
use std::time::Duration;
use clap::{arg, Parser};
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format;
use wt_blk::binary::nm_file::NameMap;
use wt_blk::binary::nm_file::parse_slim_nm;
use wt_blk::binary::{DecoderDictionary, parse_file, test_parse_dir};
use crate::fs_util::find_dict;

fn main() {
	let env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into());

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_thread_ids(false)
		.with_thread_names(true)
		.with_writer(stdout)
		.with_ansi(true)
		.without_time()
		.with_line_number(false)
		.init();

	info!("Parsing CLI args");
	let args = crate::args::Args::parse();

	info!("Capturing target folder");
	let target_dir = fs::read_dir(args.file_or_folder.as_ref().unwrap()).unwrap();
	let target_folder_raw = args.clone().file_or_folder.unwrap();

	let mut threads = vec![];

	info!("Preparing files from folder into memory");
	let prepared_files = prepare_parse_vromf_out_folder(target_dir);

	threads.push(thread::Builder::new().name("worker_thread".to_owned()).spawn(
		{
			move || {
				translate_files(&target_folder_raw,prepared_files);
			}
		}));

	for thread in threads {
		thread.unwrap().join().unwrap();
	}
}

fn prepare_parse_vromf_out_folder(dir: ReadDir) -> Vec<(String, Vec<u8>)> {
	let mut pile = vec![];
	test_parse_dir(&mut pile, dir, &AtomicUsize::new(0));
	pile
}

fn translate_files(base_path: &str, pile: Vec<(String, Vec<u8>)>) -> Vec<String> {
	info!("Reading NM file");
	let nm = fs::read(format!("{}/nm", base_path)).unwrap();
	info!("Autodetecting dict file");
	let (name, dict) = find_dict(base_path).unwrap();
	info!("Found dict at {}", name);

	info!("Preparing shared indexes");
	let frame_decoder = DecoderDictionary::copy(&dict);
	let nm = NameMap::decode_nm_file(&nm).unwrap();
	let parsed_nm = parse_slim_nm(&nm);

	let rc_nm = Rc::new(parsed_nm);
	let arced_fd = Arc::new(frame_decoder);
	info!("Parsing BLK into IR");

	let bar = Arc::new(ProgressBar::new(0));
	bar.set_style(
		ProgressStyle::with_template(" [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}").unwrap().progress_chars("#>-")
	);
	bar.set_length(pile.len() as u64);

	let out = pile.into_iter().map(|file| {
		let out = parse_file(file.1, arced_fd.clone(), &nm, rc_nm.clone());
		bar.inc(1);
		out
	}).filter_map(|x| x)
				  .collect::<Vec<_>>();
	bar.finish();

	out
}

fn autodetect_dict_location(base_path: ReadDir) -> Option<Vec<u8>> {
	for file in base_path {
		if let Ok(file) = file {
			if file.file_name().to_str().unwrap().ends_with(".dict") {
				return fs::read(file.path()).ok()
			}
		}
	}
	None
}