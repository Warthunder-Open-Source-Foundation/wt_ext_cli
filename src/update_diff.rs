

#[derive(Debug, Clone, serde::Serialize)]
pub struct Diff {
	file_name: String,
	old : FileDiff,
	new: FileDiff,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileDiff {
	sha1: String,
	time: String,
	size: usize
}

#[cfg(test)]
mod test {
	use crate::update_diff::{Diff, FileDiff};

	#[test]
	fn sample_output() {
		let diff = Diff {
			file_name: "aces.vromfs.bin".to_string(),
			old: FileDiff {
				sha1: "hexnumbersandstuff".to_string(),
				time: "sometime".to_string(),
				size: 42,
			},
			new: FileDiff {
				sha1: "newhexnumbersandstuff".to_string(),
				time: "laterthansometime".to_string(),
				size: 42,
			},
		};
		println!("{}", serde_json::to_string_pretty(&diff).unwrap());
	}
}