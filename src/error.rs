#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum CliError {
	// This error might onl occur when a flag was improperly configured and not caught by clap
	#[error("This is an internal error, it should not occur unless internal logic failed")]
	RequiredFlagMissing,

	#[error(transparent)]
	IOError(#[from] std::io::Error),

	#[error("File was missing proper descriptor")]
	MissingFileName,

	#[error("Invalid path")]
	InvalidPath,

	#[error("File does not have a valid parent folder ")]
	FileWithoutParent,

	#[error("A critical file is missing")]
	CriticalFileMissing,

	#[error("File {file_name} failed to parse DxP with: {dxp_error}")]
	DxpGrpError {
		dxp_error: wt_blk::dxp_and_grp::error::DxpGrpError,
		file_name: String,
	},

	#[error("The line {line} failed to split at a '*' char")]
	DxpGrpSplitMissing { line: String },
}
