init:
	rm -fr ./config.toml ./statics ./generates
	cargo run --features=bundled init --root .

build:
	cargo run --features=bundled build

serve:
	cargo run --features=bundled serve --port=8000
