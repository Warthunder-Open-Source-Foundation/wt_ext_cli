use std::fs;
use std::io::Read;
use std::path::Path;

const ZST_DICT_MAGIC: [u8; 4] = [0x37, 0xA4, 0x30, 0xEC];
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
					return Some((dir_entry.file_name().to_str()?.to_owned(), fs::read(dir_entry.path()).ok()?));
				}
			}
		}
	}
	None
}

#[inline]
fn is_zst_dict(file: &[u8]) -> bool {
	file.starts_with(&ZST_DICT_MAGIC)
}