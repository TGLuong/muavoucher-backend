.PHONY: run
run:
	sh scripts/run.sh

.PHONY: up
up:
	cargo run -- migrate up

.PHONY: down
down:
	cargo run -- migrate down