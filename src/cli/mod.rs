mod human_readable_output_format;

use std::str::FromStr;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::fmt::{Display, Formatter};
use crate::cli::human_readable_output_format::OutputFormat;

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
