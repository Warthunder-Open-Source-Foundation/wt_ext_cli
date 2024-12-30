use env_logger::Builder;
use log::LevelFilter;

pub fn init_logging(log_level: LevelFilter) -> color_eyre::Result<()> {
	Builder::from_default_env().filter_level(log_level).init();
	Ok(())
}
