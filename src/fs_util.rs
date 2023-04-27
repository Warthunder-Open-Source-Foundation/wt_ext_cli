use std::{
	fs,
	fs::ReadDir,
	io::Read,
	path::{Path, PathBuf},
};

use crate::error::CliError;

#[allow(dead_code)]
const ZST_DICT_MAGIC: [u8; 4] = [0x37, 0xA4, 0x30, 0xEC];
#[allow(dead_code)]
const ZST_MAGIC: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];

/// Non-recursive function for finding a dictionary file
/// Searches folder for the first match
pub fn find_dict<P: AsRef<Path>>(root: P) -> Option<(String, Vec<u8>)> {
	for f in fs::read_dir(root).ok()? {
		if let Ok(dir_entry) = f {
			if dir_entry.file_name().to_str()?.ends_with(".dict") {
				let mut file = fs::File::open(dir_entry.path()).ok()?;
				let mut buff = [0_u8; 4];
				file.read_exact(&mut buff).ok()?;
				if is_zst_dict(&buff) {
					return Some((
						dir_entry.file_name().to_str()?.to_owned(),
						fs::read(dir_entry.path()).ok()?,
					));
				}
			}
		}
	}
	None
}

#[inline(always)]
fn is_zst_dict(file: &[u8]) -> bool {
	file.starts_with(&ZST_DICT_MAGIC)
}

pub fn read_recurse_folder(
	pile: &mut Vec<(PathBuf, Vec<u8>)>,
	dir: ReadDir,
) -> Result<(), CliError> {
	// 													Yields any file
	read_recurse_folder_filtered(pile, dir, |_| true, |_| true)
}

pub fn read_recurse_folder_filtered(
	pile: &mut Vec<(PathBuf, Vec<u8>)>,
	dir: ReadDir,
	filter_name: fn(&PathBuf) -> bool, /* Mark true or false whether or not the function should yield */
	filter_content: fn(&Vec<u8>) -> bool, /* Mark true or false whether or not the function should yield */
) -> Result<(), CliError> {
	for file in dir {
		let file = file.as_ref().unwrap();
		if file.metadata().unwrap().is_dir() {
			read_recurse_folder_filtered(
				pile,
				file.path().read_dir()?,
				filter_name,
				filter_content,
			)?;
		} else {
			let path = file.path();
			if !filter_name(&path) {
				continue;
			}
			let read = fs::read(&path)?;
			if !filter_content(&read) {
				continue;
			}

			pile.push((path, read));
		}
	}
	Ok(())
}
