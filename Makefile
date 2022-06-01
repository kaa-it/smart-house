clippy:
	cargo clippy --all --all-features --tests -- -D warnings

example_report:
	cargo run --example report

