# Objection

Build server-first, highly-interactive, and beautiful web applications in Go.

...because the current web-based application development trends are worth objecting to.

> Due to an ongoing refactor, breaking changes are expected in the near future. See [#320](https://github.com/radical-ui/objection/pull/320) for current status.

## Installation

Locate the binary for your system in the release artifacts, and download it to `/usr/local/bin/objection`.

```sh
curl -L -o /usr/local/bin/objection https://github.com/radical-ui/objection/releases/download/latest/objection_darwin
```

### Install from Source

```sh
git clone https://github.com/radical-ui/objection
go build -o objection ./cli && sudo cp objection /usr/local/bin/objection && rm objection
```

## Tech Debt

- `cmd/frontend` needs some packaging
- `cmd/project_config` - `FrontendInfo` should allow edits directly to it. The whole `RemoteLocation`/`LocalLocation`
  thing with empty strings is a disaster. Add a `isRemote()` function to the struct. Additionally none of the fields
  should be public. Its functions should be mostly getters ontop of a reference to the underlying `frontendDef`.
- `bindings_path` odd. It should always default to the name of the alias, with a potential override that could be
  specified only in the config.
- there should be a spinner for aquiring a frontend lock
- logs should be written a logfile
