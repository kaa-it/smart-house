clippy:
	cargo clippy --all --all-features --tests -- -D warnings

fmt_check:
	cargo fmt --all -- --check

fmt:
	cargo fmt --all

example_report:
	cargo run --package smart-house --example report

run_server:
	cargo run --package power-switch --bin server -- -a "127.0.0.1:53453" -d "In Bathroom" -p 125.3

run_client:
	cargo run --package power-switch --bin client -- -a "127.0.0.1:53453"

run_sender:
	cargo run --package thermometer --bin sender -- -r "127.0.0.1:4444" -b "127.0.0.1:3333"

run_receiver:
	cargo run --package thermometer --bin receiver -- -r "127.0.0.1:4444" -s "127.0.0.1:3333"

