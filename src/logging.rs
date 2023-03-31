use std::io::stdout;

use tracing::Level;
use tracing_subscriber::EnvFilter;

pub fn logging() {
	let env_filter = EnvFilter::from_default_env()
		.add_directive(Level::WARN.into());

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