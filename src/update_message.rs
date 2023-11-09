#[cfg(feature = "curl-github-api")]
#[cfg(feature = "native-github-api")]
compile_error!("native-github-api and curl-github-api are mutuall exclusive");

fn print_update_msg(tag: &str) {
	warn!("Good news, a new version of this tool is available. You may download it here: https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases/tag/{}", tag)

}

use tracing::warn;
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
	use smol_timeout::TimeoutExt;
	use tracing::warn;
	use crate::GIT_TAG;

	pub fn update_message() -> Result<(), Report> {
		use smol_timeout::TimeoutExt;
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
				let current = Version::from_str(&GIT_TAG.replace("v", ""))?;
				if latest > current {
					print_update_msg(&latest_prefixed);
				} else {}
			} else {
				// Do nothing, as this means the request timed out and no message will be displayed
			}
			Ok(())
		})
	}
}

#[cfg(feature = "curl-github-api")]
mod curl_github_api {
	use std::iter::{Once, once};
	use std::process::Command;
	use std::str::FromStr;
	use std::thread::sleep;
	use std::time::Duration;
	use color_eyre::Report;

	pub fn update_message() -> Result<(), Report> {
		todo!()
	}

	fn curl_api() -> Result<serde_json::Value, Report> {
		let proc = Command::new("curl")
			.args(&[
				"-L",
				r#"-H "Accept: application/vnd.github+json""#,
				r#"-H "X-GitHub-Api-Version: 2022-11-28""#,
				"https://api.github.com/repos/Warthunder-Open-Source-Foundation/wt_ext_cli/git/refs/tags"
			])
			.spawn()?;
		let output = dbg!(proc.wait_with_output()?);
		Ok(serde_json::Value::from_str(&String::from_utf8(output.stdout)?)?)
	}

	#[cfg(test)]
	mod test {
		use crate::update_message::curl_github_api::curl_api;

		#[test]
		fn test_curl() {
			curl_api();
		}
	}
}