all:
	cargo build --release
clean:
	rm -rd target
install: all
	mkdir -p ~/.cargo/bin/
	cp target/release/jack ~/.cargo/bin/es5jack
