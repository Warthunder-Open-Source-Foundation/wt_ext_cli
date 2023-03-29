use std::fs;
use std::fs::ReadDir;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;
use wt_blk::binary::{DecoderDictionary, parse_file, test_parse_dir};
use wt_blk::binary::nm_file::NameMap;
use crate::fs_util::find_dict;

pub fn unpack_raw_blk(target_dir: ReadDir, target_folder: &str) {
	info!("Preparing files from folder into memory");
	let prepared_files = prepare_parse_vromf_out_folder(target_dir);
	translate_files(target_folder, prepared_files);

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
	let shared_nm = NameMap::from_encoded_file(&nm).unwrap();

	let rc_nm = Rc::new(shared_nm);
	let arced_fd = Arc::new(frame_decoder);
	info!("Parsing BLK into IR");

	let bar = Arc::new(ProgressBar::new(0));
	bar.set_style(
		ProgressStyle::with_template(" [{elapsed}/{eta}] [{wide_bar:.cyan/blue}] {percent}% {pos}/{len}").unwrap().progress_chars("#>-")
	);
	bar.set_length(pile.len() as u64);

	let out = pile.into_iter().map(|file| {
		let out = parse_file(file.1, arced_fd.clone(), rc_nm.clone());
		bar.inc(1);
		out
	}).filter_map(|x| x)
				  .collect::<Vec<_>>();
	bar.finish();

	out
}