objection() {
	cargo run -p objection_cli --\
		--runtime file://$(pwd)/runtime/mod.tsx \
		--engine rust \
		--bindings-path runtime_test/bindings.rs \
		--engine-url http://localhost:8000/ui $@
}

task_dev() {
	runner_parallel dev_engine dev_ui
}

task_dev_engine() {
	watchexec --restart -- runner run_engine
}

task_run_engine() {
	cargo run --package objection_runtime_test
}

task_dev_ui() {
	objection run
}

task_serve_web() {
	static-web-server --root target/objection_build --port 3000
}

task_build() {
	objection build
}

task_preview() {
	runner build
	runner_parallel run_example serve_web
}

task_release() {
	last_tag="$(git describe --abbrev=0)"
	next_tag="$(echo $1 | sed 's/--version=//')"

	echo "Moving from $last_tag to $next_tag..."

	echo "Updating all versions..."
	rg --fixed-strings --files-with-matches $last_tag | sd --fixed-strings $last_tag $next_tag
	git add -A
	git commit -m "chore: $next_tag"
	git push

	echo "Creating and pushing a new tag..."
	git tag $next_tag
	git push origin tag $next_tag

	echo "Creating a sha256 for the just-created tag..."
	git archive -o target/archive.tar.gz $next_tag
	sha256_hash="$(cat target/archive.tar.gz | sha256sum | sd "	-" "")"

	echo "Updating the homebrew formula..."
	git clone git@github.com:radical-ui/homebrew-tap.git target/tap
	cp homebrew_formula_template.rb target/tap/objection.rb
	sd --fixed-strings VERSION $next_tag target/tap/objection.rb
	sd --fixed-strings SHA256_HASH $sha256_hash target/tap/objection.rb

	cd target/tap
	git add -A
	git commit -m "chore: $next_tag"
	git push
	cd ../..

	echo "Done!!"
}
