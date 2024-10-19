use std::sync::LazyLock;

use simplelog::{
	format_description, ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger,
	TerminalMode,
};
use tokio::runtime::Runtime;

pub fn init_runtime() {
	TOKIO_RUNTIME.spawn(async {});

	let log_config = ConfigBuilder::default()
		.set_time_format_custom(format_description!(
			"[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]"
		))
		.build();

	let log_filter = LevelFilter::Info;

	let color_choice = if atty::is(atty::Stream::Stdout) {
		ColorChoice::Auto
	} else {
		ColorChoice::Never
	};

	CombinedLogger::init(vec![TermLogger::new(
		log_filter,
		log_config,
		TerminalMode::Mixed,
		color_choice,
	)])
	.unwrap();
}

pub static TOKIO_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());
