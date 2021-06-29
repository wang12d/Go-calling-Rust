build:
	cargo build --release
	cp target/release/libhello.so src/libs/
	go build -ldflags="-r $(shell pwd)/src/libs" src/hello.go