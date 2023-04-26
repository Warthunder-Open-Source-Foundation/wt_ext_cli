use std::{fs, path::PathBuf, str::FromStr};
use std::fs::create_dir_all;

use anyhow::Context;
use clap::ArgMatches;
use wt_blk::dxp;

use crate::{
	error::CliError,
	fs_util::read_recurse_folder_filtered,
};
use crate::error::CliError::DxpParse;

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

	let out_dir = args.get_one::<String>("Output directory");
	let complete_out_dir = out_dir.and_then(|p| PathBuf::from_str(p).ok());

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
		let parsed = dxp::parse_dxp(&prepared_file.1).map_err(|e|DxpParse { dxp_error: e, file_name: prepared_file.0.to_str().unwrap().to_string() })?.join("\n");
		let file_name = prepared_file
			.0
			.file_name()
			.expect("Has to be valid str")
			.to_str()
			.expect("Has to be valid str");
		let final_content = format!("{file_name}\n\n{parsed}");

		let mut final_path = prepared_file.0;
		final_path.set_extension("dxp.bin.txt");
		output.push((
			final_path,
			final_content,
		));
	}
	for file in output {
		let final_out = if let Some(out_dir) = &complete_out_dir {
			let out = out_dir.join(file.0.strip_prefix(&parsed_input_dir)?);
			create_dir_all(&out.parent().unwrap())?;
			out
		} else {
			file.0
		};
		fs::write(final_out, file.1)?;
	}

	Ok(())
}
