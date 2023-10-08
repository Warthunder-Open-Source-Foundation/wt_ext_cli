use std::process::Command;
fn main() {
	let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
	let git_hash = String::from_utf8(output.stdout).unwrap();
	println!("cargo:rustc-env=GIT_HASH={}", git_hash);

	let output = Command::new("git").args(&["describe", "--tags", "--abbrev=0"]).output().unwrap();
	let git_tag = String::from_utf8(output.stdout).unwrap();
	println!("cargo:rustc-env=GIT_TAG={}", git_tag);

	println!("cargo:rerun-if-changed=build.rs")
}