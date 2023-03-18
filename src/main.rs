mod args;

use std::borrow::Cow;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::{fs, thread};
use std::fs::ReadDir;
use std::rc::Rc;
use std::thread::{sleep, spawn};
use std::time::Duration;
use clap::Parser;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use wt_blk::binary::nm_file::NameMap;
use wt_blk::binary::nm_file::parse_slim_nm;
use wt_blk::binary::{DecoderDictionary, parse_file, test_parse_dir};

fn main() {
	let args = crate::args::Args::parse();
	let target_dir = fs::read_dir(&args.target_folder).unwrap();

	let mut threads = vec![];

	threads.push(thread::Builder::new().name("worker_threads".to_owned()).spawn(
		{
			let bar = Arc::new(ProgressBar::new(0));
			let target_folder = args.target_folder.clone();
			bar.set_style(
				ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}").unwrap()
																														.progress_chars("#>-")
			);
			bar.set_message("Parsing blk files into IR");
			move || {
				let prepared_files = prepare_parse_vromf_out_folder(target_dir);
				bar.set_length(prepared_files.len() as u64);
				translate_files(&target_folder,prepared_files, bar.clone());
				bar.finish();
			}
		}));

	for thread in threads {
		thread.unwrap().join().unwrap();
	}
	sleep(Duration::from_millis(100))
}

fn prepare_parse_vromf_out_folder(dir: ReadDir) -> Vec<(String, Vec<u8>)> {
	let mut pile = vec![];
	test_parse_dir(&mut pile, dir, &AtomicUsize::new(0));
	pile
}

fn translate_files(base_path: &str, pile: Vec<(String, Vec<u8>)>, bar: Arc<ProgressBar>) -> Vec<String> {
	let nm = fs::read(format!("{}/nm", base_path)).unwrap();
	let dict = fs::read("../wt_blk/samples/vromfs/aces.vromfs.bin_u/ca35013aabca60792d5203b0137d0a8720d1dc151897eb856b12318891d08466.dict").unwrap();

	let frame_decoder = DecoderDictionary::copy(&dict);

	let nm = NameMap::decode_nm_file(&nm).unwrap();
	let parsed_nm = parse_slim_nm(&nm);

	let rc_nm = Rc::new(parsed_nm);
	let arced_fd = Arc::new(frame_decoder);
	let out = pile.into_iter().map(|file| {
		let out = parse_file(file.1, arced_fd.clone(), &nm, rc_nm.clone());
		bar.inc(1);
		out
	}).filter_map(|x| x)
				  .collect::<Vec<_>>();
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