# mblocks - minimal dwm blocks

all:
	cargo build --release

clean:
	rm -rf target

install:
	cargo build --release
	sudo mv ./target/release/mblocks /usr/local/bin/

.PHONY: all clean install
