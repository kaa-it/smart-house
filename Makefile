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

