use std::env;
use std::io::{stderr, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{JoinHandle, spawn};
use color_eyre::eyre::{bail, Context, ContextCompat};
use color_eyre::owo_colors::colors;

pub static CAPTURE_IMAGE_CONVERTER: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ImageConverter {
	validated: bool,
	converter: Converter,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Converter {
	FFMPEG(Option<String>),
	Imagemagick(Option<String>),
}


impl Converter {
	pub fn new_from_arg(input: &str) -> color_eyre::Result<Self> {
		let c = match input {
			"imagemagick" => {Self::Imagemagick(env::var("CONVERTER_PATH").ok())}
			"ffmpeg" => {Self::FFMPEG(env::var("CONVERTER_PATH").ok())}
			&_ => {bail!("Unrecongized converter tool: {input}")}
		};
		Ok(c)
	}
}
impl AsRef<str> for Converter {
	fn as_ref(&self) -> &str {
		#[cfg(not(windows))]
		match self {
			Converter::FFMPEG(path) => {path.as_deref().unwrap_or("ffmpeg")}
			Converter::Imagemagick(path) => {path.as_deref().unwrap_or("magick")}
		}
		#[cfg(windows)]
		match self {
			Converter::FFMPEG(path) => {path.as_deref().unwrap_or("ffmpeg.exe")}
			Converter::Imagemagick(path) => {path.as_deref().unwrap_or("magick")}
		}
	}
}

impl ImageConverter {
	#[allow(unused)]
	pub fn new_ffmpeg() -> Self { ImageConverter { validated: false, converter: Converter::FFMPEG(None) } }
	#[allow(unused)]
	pub fn new_imagemagick() -> Self { ImageConverter { validated: false, converter: Converter::Imagemagick(None) } }

	pub fn new_with_converter(p: Converter) -> Self {
		ImageConverter {
			validated: false,
			converter: p,
		}
	}
	
	pub fn validate(&mut self) -> color_eyre::Result<()> {
		program_is_callable(Path::new(self.converter.as_ref()))?;
		self.validated = true;
		Ok(())
	}

	pub fn convert_and_write(&self, buf: Vec<u8>, dest: &str) -> color_eyre::Result<()> {
		let captured = || if CAPTURE_IMAGE_CONVERTER.load(Relaxed) {
			Stdio::piped()
		} else {
			Stdio::null()
		};

		let mut com = match &self.converter {
			Converter::FFMPEG(_) => {Self::spawn_ffmpeg(self.converter.as_ref(), dest, captured)}
			Converter::Imagemagick(_) => {Self::spawn_imagemagick(self.converter.as_ref(), dest, captured)}
		}?;

		let mut stdin = com.stdin.take().context("Failed to take stdin")?;

		let thread = spawn(move || {
			stdin.write_all(&buf).expect("Converter pipe died");
		});

		thread.join().unwrap();
		let output = com.wait_with_output()?;

		if !output.status.success() {
			if CAPTURE_IMAGE_CONVERTER.load(Relaxed) {
				let h = stderr();
				h.lock().write_all(&output.stderr)?;
			}
			bail!("Converter exited with {}\nRerun with CAPTURE_IMAGE_CONVERTER=true to capture stderr", output.status)
		} else {
			Ok(())
		}
	}

	fn spawn_imagemagick(program: &str, dest: &str, captured: fn() -> Stdio) -> color_eyre::Result<Child> {
		let com = Command::new(program)
			.stdin(Stdio::piped())
			.stderr(captured())
			.stdout(captured())
			.args(&[
				"convert",  "-", // - Means piped input
				"-quality", "100", // Preserve full input quality
				"-strip", // Strip unused metadata and profiles
				dest // Output to this file
			])
			.spawn()?;
		Ok(com)
	}
	fn spawn_ffmpeg(program: &str, dest: &str, captured: fn() -> Stdio) -> color_eyre::Result<Child> {
		let com = Command::new(program)
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
			.spawn()?;
		Ok(com)
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

