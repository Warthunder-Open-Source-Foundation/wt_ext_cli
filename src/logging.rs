use std::borrow::Cow;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::{fs, thread};
use std::fs::ReadDir;
use std::io::stdout;
use std::rc::Rc;
use std::thread::{sleep, spawn};
use std::time::Duration;
use clap::{arg, Parser};
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format;
use wt_blk::binary::nm_file::NameMap;
use wt_blk::binary::{DecoderDictionary, parse_file, test_parse_dir};
use crate::cli::build_command_structure;
use crate::fs_util::find_dict;

pub fn logging() {
	let env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into());

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_thread_ids(false)
		.with_thread_names(true)
		.with_writer(stdout)
		.with_ansi(true)
		.without_time()
		.with_line_number(true)
		.with_file(true)
		.init();
}