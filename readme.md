# Objection

Build server-first, interactive, and beautiful applications in Rust.

## Installation

```sh
# MacOS
brew install radical-ui/tap/objection

# Linux / Windows
# Install deno, then...
deno install -Af https://denopkg.com/radical-ui/objection/main.ts
```

## Usage

Objection works by managing a network bridge on which objects are broadcast, allowing a generic frontend application to
be managed and customized by a backend. In practice, it feels like a merge between Phenix Liveview and HTMX.

We provide high-quality frontends for [IOS](https://github.com/radical-ui/ipage),
[Android](https://github.com/radical-ui/apge), and the [web](https://github.com/todo). You can also
[build your own](/specification.md).

Before writing backend code for the frontend, types should be generated for it.

```shell
objection gen radical-ui/apage@0.1.0 types.rs
```

### Rust Backend Quick Start

Here is an example of writing a rust backend for the apage frontend using [axum](https://docs.rs/axum) as the server.

```rust
// src/main.rs
// TODO update this

use axum::{extract::State, routing::post, Json, Router};
use bindings::Label;
use objection::{handle_request, RootUi, UiResponse};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod types;

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route(
			"/ui",
			post(move |Json(body): Json<Value>| async move {
				Json(handle_request(body, |_, ui| async {
					ui.set_root_ui(Label::new("Hello, world!"));
					Ok(ui.into_reponse())
				}).await)
			}),
		)
		.layer(CorsLayer::very_permissive());

	let listener = TcpListener::bind(("localhost", 8000)).await.unwrap();
	println!("listening at http://localhost:8000");

	axum::serve(listener, app).await.unwrap();
}
```

### Preview and Deploy the Frontend

The frontend can be previewed using the `preview` command.

```shell
objection preview radical-ui/apage@0.1.0 --backend-url ws://localhost:3000/ui.ws
```

To build and deploy the frontend, use the `deploy` command.

```shell
objection deploy radical-ui/apage@0.1.0 --backend-url ws://my-backend.example.com/ui.ws
```

While it is dependent on the frontend, both of these commands tend to require a good bit of configuration. That
configuration can be supplied directly to the frontend via cli options (eg. `--backend-url` in the above examples), or
by way of an `Objection.toml`. If a configuration cannot be obtained using those methods, it will be prompted for.

## Development

The system

- [Rust](https://www.rust-lang.org/tools/install)
- [Runner](https://github.com/stylemistake/runner)
- [WatchExec](https://github.com/watchexec/watchexec)
- [Deno](https://deno.com/)
- [Ripgrep](https://github.com/BurntSushi/ripgrep)
- [sd](https://github.com/chmln/sd)
- [Static Web Server](https://github.com/static-web-server/static-web-server)

Then, start up the example project:

```sh
runner dev
```
