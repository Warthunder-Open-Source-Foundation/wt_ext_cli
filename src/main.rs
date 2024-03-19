#![feature(if_let_guard)]

use std::{env, sync::atomic::Ordering::Relaxed};

use color_eyre::eyre::Result;

use crate::{
	cli::build_command_structure,
	image_conversion::CAPTURE_IMAGE_CONVERTER,
	subcommands::branch_subcommands,
};

mod cli;
mod error;
mod fs_util;
mod image_conversion;
mod logging;
mod subcommands;
mod update_diff;
pub(crate) mod util;

pub const COMMIT_HASH: &str = env!("GIT_HASH");
pub const GIT_TAG: &str = env!("GIT_TAG");

fn main() -> Result<()> {
	env::set_var("RUST_BACKTRACE", "1");

	let enable_color = if let Ok(force_color) = env::var("FORCE_SET_COLOR") {
		force_color
			.parse::<bool>()
			.expect("FORCE_COLOR was not 'false' or 'true'")
	} else {
		if cfg!(windows) {
			false
		} else {
			true
		}
	};

	if enable_color {
		color_eyre::install()?;
	} else {
		color_eyre::config::HookBuilder::new()
			.theme(color_eyre::config::Theme::new())
			.install()?;
	}

	if let Ok(capture) = env::var("CAPTURE_IMAGE_CONVERTER") {
		let capture = capture
			.parse::<bool>()
			.expect("CAPTURE_IMAGE_CONVERTER was not 'false' or 'true'");
		CAPTURE_IMAGE_CONVERTER.store(capture, Relaxed);
	};

	// Set rayon thread names
	rayon::ThreadPoolBuilder::new()
		.thread_name(|i| format!("rayon-{i}"))
		.build_global()?;

	let command = build_command_structure().get_matches();
	branch_subcommands(command)?;

	Ok(())
}
