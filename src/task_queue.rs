use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FileTask {
	pub path:      PathBuf,
	pub task_type: TaskType,
}

#[derive(Clone, Debug)]
pub enum TaskType {
	// Simply copies contents from A to B
	Copy { destination: PathBuf },

	// Overwrite same file
	Overwrite,

	// Not sure if this is gonna be useful, does nothing in reality
	Ignore,
}
