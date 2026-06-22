use std::{
	fs,
	path::{Path, PathBuf},
	str::FromStr,
};

use clap::ArgMatches;
use color_eyre::eyre::{bail, eyre, Context, Result};
use log::info;
use wt_blk::vromf::{
	binary_container::{decode_bin_vromf, encode_bin_vromf},
	enums::{HeaderType, Packing, PlatformType},
	inner_container::{decode_inner_vromf, encode_inner_vromf},
	File,
	Metadata,
};

use crate::{error::CliError, fs_util::read_recurse_folder};

pub fn repack_vromf(args: &ArgMatches) -> Result<()> {
	info!("Mode: Repacking vromf");

	let input_path = PathBuf::from_str(
		args.get_one::<String>("input")
			.ok_or(CliError::RequiredFlagMissing)?,
	)?;
	let output_path = PathBuf::from_str(
		args.get_one::<String>("output")
			.ok_or(CliError::RequiredFlagMissing)?,
	)?;

	let (files, mut meta) = if input_path.is_dir() {
		repack_from_directory(&input_path)?
	} else {
		repack_from_vromf_file(&input_path)?
	};

	// Apply CLI overrides on top of auto-detected / TOML metadata
	if let Some(h) = parse_optional_header(args.get_one::<String>("header").map(String::as_str)) {
		meta.header_type = Some(h);
	}
	if let Some(p) = parse_optional_platform(args.get_one::<String>("platform").map(String::as_str))
	{
		meta.platform = Some(p);
	}
	if let Some(p) = parse_optional_packing(args.get_one::<String>("packing").map(String::as_str)) {
		meta.packing = Some(p);
	}
	if let Some(v) = args.get_one::<String>("version") {
		meta.version =
			Some(wt_version::Version::from_str(v).map_err(|()| eyre!("Invalid version: {v}"))?);
	}
	if *args.get_one::<bool>("digest").unwrap_or(&false) {
		meta.digest = Some(true);
	}

	// Apply final defaults for required fields
	let header_type = meta.header_type.unwrap_or(HeaderType::VRFS);
	let platform = meta.platform.unwrap_or(PlatformType::Pc);
	let packing = meta.packing.unwrap_or(Packing::PLAIN);
	let digest = meta.digest.unwrap_or(false);
	let digest_header: u8 = if digest { 0x30 } else { 0x20 };

	if header_type == HeaderType::VRFX && meta.version.is_none() {
		bail!("VRFX (extended) header requires a version. Provide --version or repack from a vromf with version info");
	}

	let final_meta = Metadata {
		header_type: Some(header_type),
		platform:    Some(platform),
		packing:     Some(packing),
		version:     meta.version,
		digest:      Some(digest),
	};

	info!(
		"Repacking {} files with {header_type:?}/{platform:?}/{packing:?}",
		files.len()
	);

	let inner_bytes = encode_inner_vromf(files, digest_header)?;
	let final_bytes = encode_bin_vromf(&inner_bytes, final_meta)?;

	if let Some(parent) = output_path.parent() {
		fs::create_dir_all(parent)?;
	}
	fs::write(&output_path, &final_bytes)?;
	info!(
		"Wrote {} bytes to {}",
		final_bytes.len(),
		output_path.display()
	);

	Ok(())
}

fn repack_from_directory(input_path: &Path) -> Result<(Vec<File>, Metadata)> {
	// Look for meta.toml inside the extracted directory
	let meta_path = input_path.join("meta.toml");

	let meta_from_file: Metadata = if meta_path.exists() {
		let toml_str = fs::read_to_string(&meta_path)?;
		let m: Metadata = toml::from_str(&toml_str)?;
		info!("Loaded metadata from: {}", meta_path.display());
		m
	} else {
		info!("No metadata TOML found, using defaults");
		Metadata::default()
	};

	// Canonicalize for consistent absolute path handling
	let base_path = input_path.canonicalize()?;

	// Walk directory from canonicalized path so file paths are also absolute
	let mut file_entries = Vec::new();
	read_recurse_folder(&mut file_entries, fs::read_dir(&base_path)?)?;

	let mut files = Vec::with_capacity(file_entries.len());
	for (path, buf) in file_entries {
		// Skip meta.toml if present in the input directory
		if path.file_name().map(|s| s.to_string_lossy().to_string()) == Some("meta.toml".to_owned())
		{
			continue;
		}
		let rel_path = path
			.strip_prefix(&base_path)
			.context(format!(
				"Path {} is not inside the input directory {}",
				path.display(),
				base_path.display()
			))?
			.to_path_buf();
		files.push(File::from_raw(rel_path, buf));
	}

	Ok((files, meta_from_file))
}

fn repack_from_vromf_file(input_path: &Path) -> Result<(Vec<File>, Metadata)> {
	let file_bytes = fs::read(input_path)?;
	let (decoded_inner, meta_from_vromf) = decode_bin_vromf(&file_bytes, false)?;
	let inner_files = decode_inner_vromf(&decoded_inner, false)?;
	info!(
		"Loaded {} files from existing vromf: {}",
		inner_files.len(),
		input_path.display()
	);
	Ok((inner_files, meta_from_vromf))
}

fn parse_optional_header(s: Option<&str>) -> Option<HeaderType> {
	match s {
		Some("vrfs") => Some(HeaderType::VRFS),
		Some("vrfx") => Some(HeaderType::VRFX),
		_ => None,
	}
}

fn parse_optional_platform(s: Option<&str>) -> Option<PlatformType> {
	match s {
		Some("pc") => Some(PlatformType::Pc),
		Some("ios") => Some(PlatformType::Ios),
		Some("android") => Some(PlatformType::Android),
		_ => None,
	}
}

fn parse_optional_packing(s: Option<&str>) -> Option<Packing> {
	match s {
		Some("plain") => Some(Packing::PLAIN),
		Some("zstd_obfs") => Some(Packing::ZSTD_OBFS),
		Some("zstd_obfs_nocheck") => Some(Packing::ZSTD_OBFS_NOCHECK),
		_ => None,
	}
}
