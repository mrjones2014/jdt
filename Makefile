typeshare:
	typeshare . --lang=typescript --output-file=./src/types.ts

test: # TODO add frontend tests here if/when they get added
	cargo test -p encoding -p viewmodel-api -p image-repo --all-features $$CARGO_OPTIONS
