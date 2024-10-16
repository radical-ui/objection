# Objection

Build beautiful, server-first applications in a data-driven way.

## Installation

```sh
# MacOS
brew install radical-ui/tap/objection

# Linux / Windows
# Install deno, then...
deno install -Af https://denopkg.com/radical-ui/objection/main.ts
```

## Usage

Objection works by syncing a series of objects between a backend and a frontend, and vice versa, allowing a generic
frontend application to be managed and customized by a backend.

### An Arbitrary Rust Backend

Here is an example of writing an arbitrary backend that returns objects that do not align to a particular frontend
schema.

You can interact with this backend via our probing tool, `objection probe ws://localhost:8000/ui.ws`.

```rust
// src/main.rs

use anyhow::Result;
use axum::Router;
use log::info;
use objection::{ObjectionService, Session};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	env_logger::init();

	let app = Router::new().route_service("/ui.ws", ObjectionService::<Instance>::new(()));

	let listener = TcpListener::bind(("localhost", 8000)).await.unwrap();
	println!("listening at ws://localhost:8000/ui.ws");

	axum::serve(listener, app).await.unwrap();
}

struct Instance;

impl Session for Instance {
	type Context = ();
	type PeerEvent = ();

	async fn create(_: Option<String>, path: String, _: &Self::Context, controller: objection::Controller<'_>) -> Result<Self> {
		info!("Creating a new session");

		controller.set_object("root_1", json!({ "hello": "this is root 1" }));
		controller.set_object("root_2", json!({ "hello": "this is root 2" }));

		Ok(Instance)
	}

	async fn watch_object(&mut self, id: &str, _: &Self::Context, controller: objection::Controller<'_>) -> Result<()> {
		info!("Requesting to watch object {id}");

		controller.set_object(id, json!({ "hello": "from the backend" }));

		Ok(())
	}
}
```

### Using a Frontend

In reality, you'll want to return objects that a frontend can understand. We provide high-quality frontends for
[IOS](https://github.com/radical-ui/ipage), [Android](https://github.com/radical-ui/apge), and the
[web](https://github.com/todo). You can also [build your own](/specification.md).

Conveniently, there are a couple tools for assisting with this process. You can generate types for the backend, that
conform to the objects that the frontend is expecting.

```shell
objection gen radical-ui/apage@0.1.0 types.rs
```

Then, instead of setting objects directly to json, you can set them to a type.

```rust
mod types;

controller.set_object("root_1", Object::Layout { .. })
```

The frontend can also be previewed using the `preview` command.

```shell
objection preview radical-ui/apage@0.1.0 --backend-url ws://localhost:3000/ui.ws
```

Additionally, the frontend can be built and deployed using the `deploy` command.

```shell
objection deploy radical-ui/apage@0.1.0 --backend-url ws://my-backend.example.com/ui.ws
```

While it is dependent on the frontend, the `preview` and `deploy` commands tend to require a good bit of configuration.
That configuration can be supplied directly to the frontend via cli options (eg. `--backend-url` in the above examples),
or by way of an `Objection.toml`. If a configuration cannot be obtained using those methods, it will be prompted for.

## Development

TODO
