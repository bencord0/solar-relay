build:
	cargo build --target arm-unknown-linux-musleabihf --release --target-dir ./target

copy: build
	scp target/arm-unknown-linux-musleabihf/release/solar-relay p0:solar-monitor/

.PHONY: build copy
