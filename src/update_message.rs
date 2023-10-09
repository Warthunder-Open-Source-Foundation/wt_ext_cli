use std::str::FromStr;
use std::time::Duration;
use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use semver::Version;
use smol_timeout::TimeoutExt;
use tracing::warn;
use crate::GIT_TAG;

pub fn update_message() -> Result<(), Report> {
	let rt = tokio::runtime::Builder::new_current_thread()
		.enable_io()
		.enable_time()
		.build()?;
	rt.block_on(async {
		let crab = octocrab::instance();
		let repo = crab.repos("Warthunder-Open-Source-Foundation", "wt_ext_cli");
		let tags = repo.list_tags()
			.send()
			.timeout(Duration::from_secs(1)).await;

		if let Some(tags) = tags {
			let latest_prefixed = tags?.items.first().context("No tags available. This is a bug")?.name.clone();
			let latest = latest_prefixed.replace("nv", "").replace("n", ""); // trim off version prefix
			let latest = Version::from_str(&latest)?;
			let current = Version::from_str(&GIT_TAG.replace("nv", "").replace("n", ""))?;
			if latest > current {
				warn!("Good news, a new version of this tool is available. You may download it here: https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases/tag/{}", latest_prefixed)
			} else {
			}
		} else {
			// Do nothing, as this means the request timed out and no message will be displayed
		}
		Ok(())
	})
}