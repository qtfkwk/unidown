build: test README.md
	cargo build --release
	cargo doc

README.md: t/README.md
	kapow $< >$@

update:
	cargo upgrade --incompatible
	cargo update

check:
	cargo outdated
	cargo audit

test:
	cargo clippy
	cargo test

clean:
	cargo clean

install:
	cargo install --path .

all: update check build

.PHONY: build update check test clean all

