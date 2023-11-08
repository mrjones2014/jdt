typeshare:
	typeshare . --lang=typescript --output-file=./src/types.ts

test: # TODO add frontend tests here if/when they get added
	cargo test
