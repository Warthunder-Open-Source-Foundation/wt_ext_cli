


use tracing_subscriber::{
	filter::{LevelFilter},
	EnvFilter,
};

pub fn init_logging(log_level: LevelFilter) {
	let env_filter = EnvFilter::from_default_env().add_directive(log_level.into());

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_thread_ids(false)
		.with_thread_names(true)
		.with_ansi(true)
		.without_time()
		.with_line_number(true)
		.with_file(true)
		.init();
}
