use anyhow::Result;
use clap::ValueEnum;

use crate::{
	build::BuildOptions,
	web::{build_web_static, run_web_static, BuildWebStaticParams, RunWebStaticParams},
	writer::{FileWriter, Writer},
};

#[derive(Debug, Clone, Copy)]
pub struct RunParams<'a> {
	pub build_options: BuildOptions<'a>,
	pub web_port: u16,
	pub reload: bool,
	pub bindings_writer: &'a FileWriter,
}

#[derive(Debug, Clone, Copy)]
pub struct BuildParams<'a> {
	pub build_options: BuildOptions<'a>,
	pub bindings_writer: &'a FileWriter,
	pub output_writer: &'a Writer,
}

#[derive(Debug, ValueEnum, Clone, Default)]
pub enum Platform {
	/// Generates a static, client-side web app. To run, start a static web server that treats `index.html` as the `/` route.
	#[default]
	WebStatic,
	/// Generates a Deno script that, when started, serves an html file at `/`. Static assets will also be served.
	/// NOTE: Currently, SSR support is limited in the fact that it does not generate html, but instead just embeds
	/// the engine-provided initial component tree, resulting in an immediate meaningful paint as soon as the JS loads.
	/// However, that the Deno script returns a full rendering of the initial component tree in html is a planned feature.
	WebSSR,
}

impl ToString for Platform {
	fn to_string(&self) -> String {
		self.to_possible_value().unwrap().get_name().to_string()
	}
}

impl Platform {
	pub async fn run(self, params: RunParams<'_>) -> Result<()> {
		match self {
			Platform::WebStatic => {
				run_web_static(RunWebStaticParams {
					build_options: params.build_options,
					web_port: params.web_port,
					reload: params.reload,
					bindings_writer: params.bindings_writer,
				})
				.await
			}
			Platform::WebSSR => todo!(),
		}
	}

	pub async fn build(self, params: BuildParams<'_>) -> Result<()> {
		match self {
			Platform::WebStatic => {
				build_web_static(BuildWebStaticParams {
					build_options: params.build_options,
					bindings_writer: params.bindings_writer,
					output_writer: params.output_writer,
				})
				.await
			}
			Platform::WebSSR => {
				todo!()
			}
		}
	}
}
