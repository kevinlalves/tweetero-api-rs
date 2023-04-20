.PHONY: boot-startup

boot-startup:
	cargo build
	cargo watch -x run
