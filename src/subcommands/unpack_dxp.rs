use std::{env, fs, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::ArgMatches;
use wt_blk::dxp;

use crate::{
	error::CliError,
	fs_util::{read_recurse_folder, read_recurse_folder_filtered},
};

pub fn unpack_dxp(args: &ArgMatches) -> Result<(), anyhow::Error> {
	let input_dir = args
		.get_one::<String>("Input directory")
		.ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir)
		.or(Err(CliError::InvalidPath))
		.context(format!(
			"The provided input directory {} is not valid",
			input_dir
		))?;
	let input_read_dir = fs::read_dir(input_dir)?;

	let mut prepared_files = vec![];
	read_recurse_folder_filtered(
		&mut prepared_files,
		input_read_dir,
		|path| {
			path.file_name()
				.expect("Bad OSstring file TODO: implement")
				.to_str()
				.unwrap()
				.ends_with(".dxp.bin")
		},
		|_| true,
	)
	.unwrap();

	let mut output = vec![];
	for prepared_file in prepared_files {
		let parsed = dxp::parse_dxp(&prepared_file.1)?.join("\n");
		let file_name = prepared_file
			.0
			.file_name()
			.expect("Has to be valid str")
			.to_str()
			.expect("Has to be valid str");
		let final_content = format!("{file_name}\n\n{parsed}");

		let appended_path = file_name.to_string() + ".txt";
		output.push((
			parsed_input_dir.join(PathBuf::from_str(&appended_path).expect("Infallible")),
			final_content,
		));
	}
	for file in output {
		fs::write(file.0, file.1)?;
	}

	Ok(())
}
