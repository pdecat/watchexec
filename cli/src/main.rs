use std::env::var;

use color_eyre::eyre::Result;
use tracing_subscriber::filter::LevelFilter;
use watchexec::{event::Event, Watchexec};

mod args;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;

	if var("RUST_LOG").is_ok() {
		tracing_subscriber::fmt::init();
	}

	let args = args::get_args()?;

	if args.is_present("verbose") {
		tracing_subscriber::fmt()
			.with_max_level(match args.occurrences_of("verbose") {
				0 => unreachable!(),
				1 => LevelFilter::INFO,
				2 => LevelFilter::DEBUG,
				_ => LevelFilter::TRACE,
			})
			.try_init()
			.ok();
	}

	let (init, runtime) = config::new(&args)?;

	let wx = Watchexec::new(init, runtime)?;

	if !args.is_present("postpone") {
		wx.send_event(Event::default()).await?;
	}

	wx.main().await??;

	Ok(())
}
