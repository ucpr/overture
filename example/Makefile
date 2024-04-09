init:
	cargo run --features=bundled init --root .

build:
	cargo run --features=bundled build

serve: build
	find ../src -type f | entr cargo run --features=bundled serve --port=8000
