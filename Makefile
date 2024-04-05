CARGO = cargo

all:

# Build the native development binaries:

target/debug/libdart_port.a: lib/dart_port/Cargo.toml $(wildcard lib/dart_port/src/*.rs) Makefile
	$(CARGO) build --package=dart_port

target/debug/libdart_port.dylib: lib/dart_port/Cargo.toml $(wildcard lib/dart_port/src/*.rs) Makefile
	$(CARGO) build --package=dart_port

# Symlink the native development binaries:

libdart_port.a: target/debug/libdart_port.a
	ln -sf $< $@

libdart_port.dylib: target/debug/libdart_port.dylib
	ln -sf $< $@

check:
	$(CARGO) test -- --nocapture --color=always

clean:
	rm -rf libdart_port.a libdart_port.dylib
	$(CARGO) clean

.PHONY: all check clean
