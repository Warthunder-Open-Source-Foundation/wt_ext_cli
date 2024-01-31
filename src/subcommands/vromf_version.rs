use std::{fs, iter::once, path::PathBuf, str::FromStr};

use clap::ArgMatches;
use serde_json::{json, Map, Value};
use wt_blk::vromf::VromfUnpacker;

use crate::error::CliError;

pub fn vromf_version(args: &ArgMatches) -> color_eyre::Result<()> {
	let input_dir = args
		.get_one::<String>("input")
		.ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir).or(Err(CliError::InvalidPath))?;

	let versions: Vec<_> = if parsed_input_dir.is_file() {
		let unpacker =
			VromfUnpacker::from_file((parsed_input_dir.clone(), fs::read(&parsed_input_dir)?))?;
		vec![(
			parsed_input_dir
				.file_name()
				.unwrap()
				.to_string_lossy()
				.to_string(),
			unpacker.latest_version()?,
		)]
	} else {
		let dir = parsed_input_dir.read_dir()?;
		let mut versions = vec![];
		for file in dir {
			let p = file?.path();
			let unpacker = VromfUnpacker::from_file((p.clone(), fs::read(&p)?))?;
			versions.push((
				p.file_name().unwrap().to_string_lossy().to_string(),
				unpacker.latest_version()?,
			));
		}
		versions
	};
	let json = Value::Array(
		versions
			.into_iter()
			.map(|e| {
				Value::Object(Map::from_iter(once((
					e.0,
					json!(e.1.map(|e| e.to_string())),
				))))
			})
			.collect(),
	);
	println!("{}", serde_json::to_string_pretty(&json)?);

	Ok(())
}
