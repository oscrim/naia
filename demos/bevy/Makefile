docker:
	cd server; \
	cargo +nightly build --features use-webrtc --release --target-dir target/

	cp server/target/release/naia-bevy-server-demo server-demo
	docker build . -t naia-docker-example
	rm server-demo
