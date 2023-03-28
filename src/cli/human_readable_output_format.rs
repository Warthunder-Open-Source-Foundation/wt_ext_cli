use std::fmt::{Display, Formatter};
use std::str::FromStr;
use clap::ValueEnum;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
	/// Intermediate representation as used internally, formatted to Json
	IntermediateRepresentation,
	/// Json in the same format the way existing data mining tools export to
	Json,
	/// BLK text as the game uses internally
	Blk,
}

impl Display for OutputFormat {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self  {
			OutputFormat::IntermediateRepresentation => write!(f, "Json"),
			OutputFormat::Json => write!(f, "Json"),
			OutputFormat::Blk => write!(f, "Json"),
		}
	}
}

impl FromStr for OutputFormat {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"IR" => Ok(OutputFormat::IntermediateRepresentation),
			"BLK" => Ok(OutputFormat::Blk),
			"JSON" => Ok(OutputFormat::Json),
			_ => {
				Err(())
			}
		}
	}
}
