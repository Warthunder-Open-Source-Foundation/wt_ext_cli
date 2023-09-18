use std::io::stdout;
use time::OffsetDateTime;
use tracing_appender::rolling;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_logging(log_level: LevelFilter, maybe_write_log_to_file: Option<&String>) -> color_eyre::Result<()> {
	let env_filter = EnvFilter::from_default_env().add_directive(log_level.into());

	let sub = tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_thread_ids(false)
		.with_thread_names(true)
		.with_ansi(true)
		.without_time()
		.with_line_number(true)
		.with_file(true);

	if let Some(path) = maybe_write_log_to_file {
		let debug_file = rolling::never(
			path,
			format!(
				"wt_ext_cli_{}.log",
				OffsetDateTime::now_local()?.unix_timestamp()),
		).with_filter(|_| true);
		sub.with_writer(stdout.and(debug_file)).init();
	} else {
		sub.init()
	}
	Ok(())
}
