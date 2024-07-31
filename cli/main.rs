mod bundle;
mod collect;
mod convert;
mod diagnostic;
mod gen_js_entry;
mod gen_rust;
mod inspect;
mod module_loader;

use anstyle::{AnsiColor, Color as AnsColor, Style};
use anyhow::{bail, Context, Result};
use bundle::Bundler;
use clap::{builder::Styles, Parser, Subcommand, ValueEnum};
use collect::Collection;
use colored::{Color, Colorize};
use deno_graph::source::MemoryLoader;
use diagnostic::DiagnosticList;
use env_logger::Env;
use gen_js_entry::gen_js_entry;
use gen_rust::RustGen;
use inspect::Inspector;
use log::{error, info, Level};
use module_loader::load_modules;
use std::{env::current_dir, io::Write, path::PathBuf, process::exit};
use tokio::{
	fs::{read_to_string, write},
	runtime::Builder,
};
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
}

impl Engine {
	fn get_bindings(&self, collection: &Collection) -> String {
		match self {
			Self::Rust => {
				let mut gen = RustGen::new(collection);
				gen.gen();
				info!("Generated rust engine bindings");

				let output = gen.get_output();
				info!("Formatted rust engine bindings");

				output
			}
		}
	}
}

#[derive(Parser, Debug, Clone)]
#[command(styles = get_styles())]
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
	let base_url = Url::from_directory_path(current_dir().context("Failed to get the current working directory")?).unwrap();
	let runtime_url = base_url.join(&args.runtime).context("Failed to resolve runtime entry")?;
	let mut diagnostic_list = DiagnosticList::new();
	let mut memory_loader = MemoryLoader::default();
	let mut bundler = Bundler::default();
	let mut collection = Collection::default();

	load_modules(&runtime_url, &mut memory_loader, &mut bundler).await?;
	info!("Loaded runtime");

	collection.collect(&runtime_url, &memory_loader).await?;
	collection.check_components();

	let errors = collection.get_errors();
	let error_count = errors.len();

	for error in errors {
		error!("{:?}", error);
	}

	if error_count > 0 {
		bail!(
			"could not mount runtime due to {} previous error{}",
			error_count,
			if error_count == 1 { "" } else { "s" }
		);
	}

	info!("Mounted runtime");

	let inspector = Inspector::new(&collection);
	inspector.inspect(&mut diagnostic_list);

	diagnostic_list.flush("validate runtime")?;
	info!("Validated runtime");

	let response = bundler.bundle(gen_js_entry(&runtime_url, &args.engine_url, &collection)?).await?;
	info!("Bundled runtime");

	write("target/web/index.html", read_to_string("platform/web/index.html").await.unwrap())
		.await
		.unwrap();
	write("target/web/bundle.js", response).await?;

	info!("Wrote runtime platform to target/bundle.js");

	let bindings = args.engine.get_bindings(&collection);
	write(&args.bindings_path, bindings).await?;

	info!("Wrote rust engine bindings to {}", args.bindings_path.into_os_string().into_string().unwrap());

	Ok(())
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
