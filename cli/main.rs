mod asset_loader;
mod build;
mod bundle;
mod collect;
mod convert;
mod diagnostic;
mod engine;
mod gen_rust;
mod inspect;
mod module_loader;
mod platform;
mod tcp_watcher;
mod web;
mod writer;

use anstyle::{AnsiColor, Color as AnsColor, Style};
use anyhow::{Context, Result};
use build::BuildOptions;
use clap::{builder::Styles, Parser, Subcommand};
use colored::{Color, Colorize};
use engine::Engine;
use env_logger::Env;
use log::{error, Level};
use platform::{BuildParams, Platform, RunParams};
use std::{
	env::{self, current_dir},
	io::Write,
	path::PathBuf,
	process::exit,
};
use tokio::runtime::Builder;
use url::Url;
use writer::Writer;

const VERSION: &str = "0.7.1";

#[derive(Parser, Debug, Clone)]
#[command(styles = get_styles(), version(VERSION))]
struct Command {
	/// The runtime to use. Must be a url.
	#[arg(long, default_value_t = Url::parse(&format!("https://raw.githubusercontent.com/radical-ui/objection/blob/{VERSION}/runtime/mod.tsx")).unwrap())]
	runtime: Url,

	/// The platform to build for. Defaults to `web`.
	#[arg(long, default_value_t = Default::default())]
	platform: Platform,

	/// The engine that the componet trees will be built in.
	#[arg(long, default_value_t = Default::default())]
	engine: Engine,

	/// The path that engine bindings should be written to.
	#[arg(long)]
	bindings_path: PathBuf,

	/// The url that the engine will be running at. Can be a websocket or http url.
	#[arg(long)]
	engine_url: Url,

	/// The type of operation to run
	#[command(subcommand)]
	operation: Operation,

	/// The deno script to use for bundling the runtime. Primarily useful if one wants to test a modified version of the default bundler.
	#[arg(long, default_value_t = Url::parse(&format!("https://raw.githubusercontent.com/radical-ui/objection/blob/{VERSION}/bundle/mod.ts")).unwrap())]
	bundler: Url,

	/// The directory that is used for caching incremental build artifacts and outputing platform-specific applications
	#[arg(long, default_value = "target")]
	target_dir: PathBuf,

	/// The directory that is used for caching artifacts that are not project specific, such as resources downloaded
	#[arg(long, default_value = get_default_cache_dir())]
	cache_dir: PathBuf,
}

#[derive(Subcommand, Debug, Clone)]
enum Operation {
	/// Run the application using the configured runtime (see --runtime) and platform (see --platform). Engine is expected to be
	/// already running at the configured engine url (see --engine-url)
	///
	/// If the engine is restarted, the generated client will hot-reload with the changes.
	Run {
		/// What port to use for when running the web and web-ssr platforms
		#[arg(long, default_value_t = 3000)]
		web_port: u16,

		/// Do not hot-reload the generated client if the engine is restarted.
		#[arg(long)]
		no_reload: bool,
	},
	/// Build the configured runtime (see --runtime) for the configured platform (see --platform), which, when run, will access the
	/// engine at the configured engine url (see --engine-url). Code will be written to the configured output dir (see --out-dir).
	Build,
}

fn main() {
	env_logger::Builder::from_env(Env::default().default_filter_or("info"))
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
			Err(err) => {
				error!("{:?}", err);
				exit(1);
			}
		}
	});
}

async fn main_async() -> Result<()> {
	let args = Command::parse();
	let build_options = BuildOptions {
		bundler: &args.bundler,
		runtime: &args.runtime,
		engine_url: &args.engine_url,
		engine: args.engine,
	};
	let bindings_writer = Writer::new(current_dir().context("failed to get the current working directory")?).into_file_writer(args.bindings_path);
	let cache_writer = Writer::new(args.cache_dir);
	let target_writer = Writer::new(args.target_dir);

	match args.operation {
		Operation::Run { web_port, no_reload } => {
			args.platform
				.run(RunParams {
					build_options,
					web_port,
					reload: !no_reload,
					bindings_writer: &bindings_writer,
					target_writer: &target_writer,
					cache_writer: &cache_writer,
				})
				.await
		}
		Operation::Build => {
			args.platform
				.build(BuildParams {
					build_options,
					bindings_writer: &bindings_writer,
					target_writer: &target_writer,
					cache_writer: &cache_writer,
				})
				.await
		}
	}
}

fn get_styles() -> Styles {
	Styles::styled()
		.usage(Style::new().bold().underline().fg_color(Some(AnsColor::Ansi(AnsiColor::Yellow))))
		.header(Style::new().bold().underline().fg_color(Some(AnsColor::Ansi(AnsiColor::Yellow))))
		.literal(Style::new().fg_color(Some(AnsColor::Ansi(AnsiColor::Green))))
		.invalid(Style::new().bold().fg_color(Some(AnsColor::Ansi(AnsiColor::Red))))
		.error(Style::new().bold().fg_color(Some(AnsColor::Ansi(AnsiColor::Red))))
		.valid(Style::new().bold().underline().fg_color(Some(AnsColor::Ansi(AnsiColor::Green))))
		.placeholder(Style::new().fg_color(Some(AnsColor::Ansi(AnsiColor::White))))
}

fn get_default_cache_dir() -> String {
	let home = env::var("HOME").expect("Failed to find the HOME env variable");
	let segment = ".cache/objection";

	if home.ends_with("/") {
		format!("{home}{segment}")
	} else {
		format!("{home}/{segment}")
	}
}
