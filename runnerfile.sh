S="/tmp/objection_dev_socket"

task_dev() {
  watchexec --restart -- runner run
}

task_serve_web() {
  static-web-server --root target/web --port 3000
}

task_run_example() {
  cargo run --package example
}

task_run() {
  runner build
  runner_parallel run_example serve_web
}

task_build() {
  cargo run -p objection_cli --\
    --runtime file://$(pwd)/runtime/mod.tsx \
    --engine rust \
    --bindings-path example/bindings.rs \
    --engine-url http://localhost:8000/ui run
}
