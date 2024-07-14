mod collect;
mod module_loader;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use collect::Collection;
use colored::{Color, Colorize};
use env_logger::Env;
use log::{error, Level, LevelFilter};
use std::{env::current_dir, io::Write, path::PathBuf};
use tokio::runtime::Builder;
use url::Url;

#[derive(Debug, ValueEnum, Clone, Default)]
enum Platform {
	Ios,
	Android,
	Macos,
	Linux,
	Windows,
	#[default]
	Web,
}

impl ToString for Platform {
	fn to_string(&self) -> String {
		self.to_possible_value().unwrap().get_name().to_string()
	}
}

#[derive(Debug, ValueEnum, Clone)]
enum Engine {
	Rust,
	Go,
	Typescript,
}

#[derive(Parser, Debug, Clone)]
struct Command {
	/// The runtime to use. Can be a path or a full url
	#[arg(long)]
	runtime: String,

	/// The platform to build for. Defaults to `web`.
	#[arg(long, default_value_t = Default::default())]
	platform: Platform,

	/// The engine that the componet trees will be built in.
	#[arg(long)]
	engine: Engine,

	/// The path that engine bindings should be written to.
	#[arg(long)]
	bindings_path: PathBuf,

	/// The url that the engine will be running at. Can be a websocket or http url.
	#[arg(long, default_value_t = Url::parse("http://localhost:5000").unwrap())]
	engine_url: Url,

	/// The type of operation to run
	#[command(subcommand)]
	operation: Operation,
}

#[derive(Subcommand, Debug, Clone)]
enum Operation {
	/// Run the application using the configured runtime (see --runtime) and platform (see --platform). Engine is expected to be
	/// already running at the configured engine url
	Run {
		/// Watch the runtime code and reload application if it is updated. Should only be necessary if you are working on the
		/// runtime.
		#[arg(long)]
		watch_runtime: bool,

		/// Watch the engine and reload if it is restarted.
		#[arg(long)]
		reload: bool,
	},
	/// Build the configured runtime (see --runtime) for the configured platform (see --platform), which, when run, will access the
	/// engine at the configured engine url (see --engine-url). Each platform and runtime will be nested inside the folder.
	// For example, if you set this to "out", a build with "--runtime=preact --platform=web" would be written to `out/web_preact`
	Build {
		#[arg(long, default_value_t = String::from("target"))]
		out_dir: String,
	},
}

fn main() {
	env_logger::Builder::from_env(Env::default().default_filter_or("info"))
		.filter_level(LevelFilter::Info)
		.format(|buf, record| {
			writeln!(
				buf,
				"{}{} {}",
				record.level().to_string().to_lowercase().bold().color(match record.level() {
					Level::Error => Color::Red,
					Level::Warn => Color::Yellow,
					Level::Info => Color::Green,
					Level::Debug => Color::Blue,
					Level::Trace => Color::Cyan,
				}),
				":".bold().white(),
				record.args()
			)
		})
		.try_init()
		.unwrap();

	Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
		match main_async().await {
			Ok(_) => (),
			Err(err) => error!("{:?}", err),
		}
	});
}

async fn main_async() -> Result<()> {
	let args = Command::parse();
	let base_url = Url::from_directory_path(current_dir().context("Failed to get the current working directory")?).unwrap();
	let runtime_url = base_url.join(&args.runtime).context("Failed to resolve runtime")?;

	let mut collection = Collection::default();
	collection.collect(&runtime_url).await?;
	collection.check_components();

	for error in collection.get_errors() {
		error!("{:?}", error);
	}

	Ok(())
}
