cargo-watch -q -c -w src/ -x run

cargo watch -q -c -w tests/* -x "test quick_dev -- --nocapture"