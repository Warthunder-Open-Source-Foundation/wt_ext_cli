use std::borrow::Cow;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::thread::{sleep, spawn};
use std::time::Duration;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};

fn main() {
	let bar = Arc::new(ProgressBar::new(123));
	bar.set_style(
		ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}").unwrap()
			.progress_chars("#>-")
	);

	bar.tick();

	let mut threads = vec![];

	threads.push(spawn(
		{
			let bar = bar.clone();
			move || {
				for _ in 0..123 {
					sleep(Duration::from_millis(100));
					bar.inc(1);
				}
			}
		}));

	for thread in threads {
		thread.join().unwrap();
	}
	bar.finish();
	sleep(Duration::from_millis(100))
}
