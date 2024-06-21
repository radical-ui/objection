use clap::{Parser, Subcommand, ValueEnum};
use tokio::runtime::Builder;

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

#[derive(Debug, ValueEnum, Clone)]
enum Engine {
	Rust,
	Go,
	Typescript,
}

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Command {
	/// The runtime to use. For more information, run `quack explain runtime`. Defaults to `preact`.
	#[arg(long, default_value_t = String::from("runtime"))]
	runtime: String,

	/// The platform to build for. For more information, run `quack explain platform`. Defaults to `web`.
	#[arg(long)]
	platform: Option<Platform>,

	/// The engine that will be used to generated the component trees. If specified, bindings will be generated for it.
	#[arg(long)]
	engine: Option<Engine>,

	/// The path that engine bindings should be written to. Has no effect if `--engine` is not specified also.
	#[arg(long, default_value_t = String::from("http://localhost:5000"))]
	bindings_path: String,

	/// The url that the engine will be running at. Can be a websocket or http url. For an explanation of the interface that
	/// the engine will need to implement, run `quack explain engine`
	#[arg(long)]
	engine_url: Option<String>,

	/// The icon that will be used as a favicon on web, and launch icon on the other platforms.
	#[arg(long)]
	app_icon: Option<String>,

	/// The type of operation to run
	#[command(subcommand)]
	operation: Operation,
}

#[derive(Subcommand, Debug, Clone)]
enum Operation {
	/// Explain a concept about this tool
	Explain {
		/// The concept to explain
		#[command(subcommand)]
		concept: ExplanationConcept,
	},
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

#[derive(Subcommand, Debug, Clone)]
enum ExplanationConcept {
	Engine,
	Runtime,
	Platform,
}

fn main() {
	Builder::new_current_thread().enable_all().build().unwrap().block_on(main_async());
}

async fn main_async() {
	let args = Command::parse();

	dbg!(&args);
	// doc(&args.runtime).await;

	println!("Hello, world!");
}
