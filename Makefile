build:
	cargo build --release

install:
	cp target/release/rofi-recent ~/.local/bin/

all: build install
