#[cfg(feature = "curl-github-api")]
#[cfg(feature = "native-github-api")]
compile_error!("native-github-api and curl-github-api are mutually exclusive");

fn print_if_new(latest: Version) -> Result<(), Report> {
	let current = Version::from_str(&GIT_TAG.replace("v", ""))?;
	if latest > current {
		warn!("Good news, a new version of this tool is available. You may download it here: https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases/tag/v{}", latest)
	} else {}
	Ok(())
}

use std::str::FromStr;
use color_eyre::Report;
use semver::Version;
use tracing::warn;
use crate::GIT_TAG;

#[cfg(feature = "curl-github-api")]
pub use curl_github_api::update_message;

#[cfg(feature = "native-github-api")]
pub use native_github_api::update_message;

#[cfg(feature = "native-github-api")]
mod native_github_api {
	use std::str::FromStr;
	use std::time::Duration;
	use color_eyre::eyre::ContextCompat;
	use color_eyre::Report;
	use semver::Version;
	use crate::update_message::print_if_new;
	use smol_timeout::TimeoutExt;

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
				let latest = latest_prefixed.replace("v", ""); // trim off version prefix
				let latest = Version::from_str(&latest)?;
				print_if_new(latest)?;
			} else {
				// Do nothing, as this means the request timed out and no message will be displayed
			}
			Ok(())
		})
	}
}

#[cfg(feature = "curl-github-api")]
mod curl_github_api {
	#[derive(Debug, serde::Deserialize)]
	struct TaggedCommit {
		#[serde(rename(deserialize = "ref"))]
		reference: String,
	}

	use std::process::Command;
	use std::str::FromStr;
	use color_eyre::eyre::ContextCompat;
	use color_eyre::Report;
	use semver::Version;
	use crate::update_message::print_if_new;

	pub fn update_message() -> Result<(), Report> {
		let json = curl_api()?;
		let latest = json.into_iter()
			.map(|e|e.reference.split("/").nth(2).map(|e|e.to_owned()).context("Invalid refs, missing slashes"))
			.last().context("Zero refs found")??;
		print_if_new(Version::from_str(&latest.replace("v", ""))?)?;
		Ok(())
	}

	fn curl_api() -> Result<Vec<TaggedCommit>, Report> {
		let output = Command::new("curl")
			.args(&[
				"-L",
				"--silent",
				r#"-H "Accept: application/vnd.github+json""#,
				r#"-H "X-GitHub-Api-Version: 2022-11-28""#,
				"https://api.github.com/repos/Warthunder-Open-Source-Foundation/wt_ext_cli/git/refs/tags"
			])
			.output()?;
		Ok(serde_json::from_slice::<Vec<TaggedCommit>>(&output.stdout)?)
	}
}