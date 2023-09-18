use std::env;
use std::io::stdout;
use color_eyre::eyre::ContextCompat;
use time::OffsetDateTime;
use tracing::{info};
use tracing_appender::rolling;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_logging(log_level: LevelFilter, mut maybe_write_log_to_file: Option<String>, crashlog: bool) -> color_eyre::Result<()> {
	let env_filter = EnvFilter::from_default_env().add_directive(if crashlog { LevelFilter::TRACE } else { log_level }.into());

	let sub = tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_thread_ids(false)
		.with_thread_names(true)
		.with_ansi(!crashlog && !cfg!(windows)) // Only log ansi-colors when not on windows, and not in crash mode
		.without_time()
		.with_line_number(true)
		.with_file(true);

	if crashlog {
		maybe_write_log_to_file = Some(env::current_dir()?.to_str().context("Current dir is not a valid UTF-8 string")?.to_owned());
	}

	if let Some(path) = maybe_write_log_to_file {
		let debug_file = rolling::never(
			&path,
			format!(
				"wt_ext_cli_{}.log",
				OffsetDateTime::now_local()?.unix_timestamp()),
		);
		sub.with_writer(stdout.and(debug_file)).init();
	} else {
		sub.init()
	}
	info!("Logging initialized");
	Ok(())
}
