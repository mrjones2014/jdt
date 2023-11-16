typeshare:
	typeshare . --lang=typescript --output-file=./src/types.ts

test: # TODO add frontend tests here if/when they get added
	cargo test

check: # TODO add frontend checks like eslint, prettier, etc.
	cargo check

# NOTE the WEBKIT_DISABLE_COMPOSITING_MODE=1 fixes a bug on NixOS and macOS
# where the app is running and interactive but the renderer is just a blank screen
# see: https://github.com/tauri-apps/tauri/issues/5143
# Specifically this comment: https://github.com/tauri-apps/tauri/issues/5143#issuecomment-1311815517
run:
	WEBKIT_DISABLE_COMPOSITING_MODE=1 cargo tauri dev

docs:
	# If running in CI, only generate the files, don't try to open web browser
	if [ "${CI}" != "" ]; then cargo doc --no-deps; else cargo doc --no-deps --open; fi
