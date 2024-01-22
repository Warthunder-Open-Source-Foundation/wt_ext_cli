use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use clap::ArgMatches;
use wt_blk::vromf::VromfUnpacker;
use wt_version::Version;
use crate::error::CliError;

pub fn vromf_version(args: &ArgMatches) -> color_eyre::Result<()> {
	let input_dir = args
		.get_one::<String>("input")
		.ok_or(CliError::RequiredFlagMissing)?;
	let parsed_input_dir = PathBuf::from_str(&input_dir).or(Err(CliError::InvalidPath))?;

	let versions: Vec<_> = if parsed_input_dir.is_file() {
		let unpacker = VromfUnpacker::from_file((parsed_input_dir.clone(), fs::read(&parsed_input_dir)?))?;
		vec![(parsed_input_dir.file_name().unwrap().to_string_lossy().to_string(), unpacker.latest_version().ok())]
	} else {
		let dir = parsed_input_dir.read_dir()?;
		let mut versions = vec![];
		for file in dir {
			let p = file?.path();
			let unpacker = VromfUnpacker::from_file((p.clone(), fs::read(&p)?))?;
			versions.push((p.file_name().unwrap().to_string_lossy().to_string(), unpacker.latest_version().ok()));
		}
		versions
	}.into_iter().map(|(mut i,e)|{
		i.push(' ');
		i.push_str(&e.map(|e|e.to_string()).unwrap_or("0.0.0.0".to_owned()));
		i
	}).collect();
	println!("{}", serde_json::to_string_pretty(&versions)?);

	Ok(())
}