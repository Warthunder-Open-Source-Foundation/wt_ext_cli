#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Diff {
	file_name: String,
	old:       FileDiff,
	new:       FileDiff,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileDiff {
	sha1: String,
	time: String,
	size: usize,
}

#[cfg(test)]
mod test {
	use std::{fs, io::BufRead};

	use crate::update_diff::{Diff, FileDiff};

	#[test]
	fn sample_output_diff() {
		let diff = Diff {
			file_name: "aces.vromfs.bin".to_string(),
			old:       FileDiff {
				sha1: "hexnumbersandstuff".to_string(),
				time: "sometime".to_string(),
				size: 42,
			},
			new:       FileDiff {
				sha1: "newhexnumbersandstuff".to_string(),
				time: "laterthansometime".to_string(),
				size: 42,
			},
		};
		println!("{}", serde_json::to_string_pretty(&diff).unwrap());
	}

	#[test]
	fn parse_yup_old_sample() {
		let yup = fs::read("./test_data/dev_2.23.1.9.yup").unwrap();
		let s = String::from_utf8_lossy(&yup).to_string();
		let split = s.split(":").collect::<Vec<_>>();
		fs::write("test_data/yup.txt", split.join("\n")).unwrap();
	}
}
