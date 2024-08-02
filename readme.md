# Objection

Build server-first, highly-interactive, and beautiful web applications in Rust and Go.

...because the current web-based application development trends are worth objecting to.

## Usage

Objection works by generating a network bridge, allowing a series of typescript components (referred to as the runtime) to be managed by your
backend (referred to as the engine). In practice, it feels like a merge between Phenix Liveview and HTMX.

The default runtime is located in the `runtime` folder, but you can create and use your own.

### Rust Engine

The runtime can be started, and Rust bindings generated, by using the following command:

```sh
objection --engine rust --bindings-path src/bindings.rs --engine-url http://localhost:8000/ui run
```

The corresponding Rust engine can be written like so:

```rust
// src/main.rs

use axum::{extract::State, routing::post, Json, Router};
use bindings::Label;
use objection::{handle_request, RootUi, UiResponse};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod bindings;

#[tokio::main]
async fn main() {
	let port = 8000;

	let app = Router::new()
		.route(
			"/ui",
			post(move |State(queue): State<_>, Json(body): Json<Value>| async move {
				Json(handle_request(body, |_, ui| async {
          ui.set_root_ui(Label::new("Hello, world!"));
          Ok(ui.into_reponse())
        }).await)
			}),
		)
		.layer(CorsLayer::very_permissive())
		.with_state(queue);

	let listener = TcpListener::bind(("localhost", 3000)).await.unwrap();
	println!("listening at http://localhost:3000");

	axum::serve(listener, app).await.unwrap();
}
```

That should do it. After starting the engine, navigate to `http://localhost:3000`.

## Deveopment

You'll want to make sure that you have development dependencies installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Runner](https://github.com/stylemistake/runner)
- [WatchExec](https://github.com/watchexec/watchexec)
- [Deno](https://deno.com/)
- [Static Web Server](https://github.com/static-web-server/static-web-server)

Then, start up the example project:

```sh
runner dev
```
