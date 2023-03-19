use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone)]
#[command(name = "wt_ext_cli")]
#[command(author = "FlareFlo")]
#[command(about = "CLI tooling to extract, inspect and manipulate WT files")]
#[command(version)]
#[command(next_line_help = true)]
#[command(color =  clap::ColorChoice::Always)]
pub struct Args {
	// This argument is set, when the binary is called into without any arguments except a folder, this usually occurs via drag and drop onto the .exe on windows
	/// Path to a folder of files, or a single file
	pub file_or_folder: Option<String>,

	/// Folder from raw vromfs output
	#[arg(short = 'i', long)]
	pub vromf_raw: Option<PathBuf>,

	/// Output folder name or output file name
	#[arg(short = 'o' , long)]
	pub output: Option<PathBuf>,

	/// Type of output the program should yield
	#[arg(short = 'f', long, default_value = "json")]
	pub output_format: OutputFormat,
}

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