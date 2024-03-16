use std::io::{stderr, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{JoinHandle, spawn};
use color_eyre::eyre::{bail, Context, ContextCompat};
use color_eyre::owo_colors::colors;
use crate::ffmpeg::ffmpeg_names::FFMPEG;

#[cfg(windows)]
mod ffmpeg_names {
	pub const FFMPEG: &str = "ffmpeg.exe";
}
#[cfg(not(windows))]
mod ffmpeg_names {
	pub const FFMPEG: &str = "ffmpeg";
}

pub static CAPTURE_FFMPEG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Ffmpeg {
	validated: bool,
	path: Option<PathBuf>,
}

impl Ffmpeg {
	pub fn new() -> Self { Ffmpeg { validated: true, path: None } }

	pub fn new_with_path(p: String) -> Self {
		Ffmpeg {
			validated: false,
			path: Some(PathBuf::from(p)),
		}
	}

	pub fn path(&self) -> &Path {
		self.path.as_deref().unwrap_or(Path::new(FFMPEG))
	}
	
	pub fn validate(&mut self) -> color_eyre::Result<()> {
		self.path.as_deref().map(program_is_callable).transpose()?;
		self.validated = true;
		Ok(())
	}

	pub fn convert_and_write(&self, buf: Vec<u8>, dest: &str) -> color_eyre::Result<()> {
		let captured = || if CAPTURE_FFMPEG.load(Relaxed) {
			Stdio::piped()
		} else {
			Stdio::null()
		};

		let mut com = Command::new("ffmpeg")
			.stdin(Stdio::piped())
			.stderr(captured())
			.stdout(captured())
			.args(&[
				"-hwaccel", "auto", // Optionally enables HWaccel when available
				"-i", "-", // Takes stdin as source
				"-f", "image2pipe", // Take pipe as source
				"-y", // Overwrites any existing file and creates if necessary
				dest // Output to this file
			])
			.spawn()
			.unwrap();

		let mut stdin = com.stdin.take().context("Failed to take stdin")?;

		let thread = spawn(move || {
			stdin.write_all(&buf).expect("FFMPEG pipe died");
		});

		thread.join().unwrap();
		let output = com.wait_with_output()?;

		if !output.status.success() {
			if CAPTURE_FFMPEG.load(Relaxed) {
				let h = stderr();
				h.lock().write_all(&output.stderr)?;
			}
			bail!("FFmpeg exited with {}", output.status)
		} else {
			Ok(())
		}
	}
}

fn program_is_callable(name: &Path) -> color_eyre::Result<()> {
	Command::new(name)
		.arg("-version")
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
		.with_context(|| format!("failed to call {}, is it in path?", name.display()))?
		.wait()
		.with_context(|| format!("{} exited with a non-zero error code", name.display()))
		.map(|_| ())
}

