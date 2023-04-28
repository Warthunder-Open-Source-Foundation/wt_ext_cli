use std::{fs, fs::create_dir_all, io::Read, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::ArgMatches;
use wt_blk::dxp_and_grp::{dxp, parse_buffered};

use crate::{
	fs_util::{fd_recurse_folder_filtered, read_recurse_folder_filtered},
};
use crate::error::CliError;
use crate::error::CliError::{DxpGrpError, DxpGrpSplitMissing};

pub fn unpack_dxp_and_grp(args: &ArgMatches) -> Result<(), anyhow::Error> {
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

	let keep_suffix = args.get_flag("Keep suffix");

	let mut prepared_files = vec![];
	fd_recurse_folder_filtered(&mut prepared_files, input_read_dir, |path| {
		let fname= path.file_name()
			.expect("Bad OSstring file TODO: implement")
			.to_str()
			.unwrap();
		fname.ends_with(".dxp.bin") || fname.ends_with(".grp")

	})
	.unwrap();

	for mut prepared_file in prepared_files {
		let mut dxp_or_grp = parse_buffered(&prepared_file.1).map_err(|e| DxpGrpError {
			dxp_error: e,
			file_name: prepared_file.0.to_str().unwrap().to_string(),
		})?;
		if !keep_suffix {
			for name in &mut dxp_or_grp {
				*name = name
					.split("*")
					.next()
					.ok_or(DxpGrpSplitMissing {
						line: name.to_string(),
					})?
					.to_owned();
			}
		}

		let parsed = dxp_or_grp.join("\n");
		let file_name = prepared_file.0
			.as_path()
			.file_name()
			.expect("Has to be valid str")
			.to_str()
			.expect("Has to be valid str");
		let final_content = format!("folder {file_name}\n\n{parsed}");

		let mut final_path = prepared_file.0.clone();
		if file_name.ends_with("grp") {
			final_path.set_extension("grp.txt");
		} else {
			final_path.set_extension("txt");
		}
		let file = (final_path, final_content);

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
