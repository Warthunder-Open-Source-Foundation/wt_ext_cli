
#[derive(Debug, thiserror::Error)]
pub enum CliError {
	// This error might onl occur when a flag was improperly configured and not caught by clap
	#[error("This is an internal error, it should not occur unless internal logic failed")]
	RequiredFlagMissing,

	#[error(transparent)]
	IOError(#[from] std::io::Error),

	#[error("File was missing proper descriptor")]
	MissingFileName,
}