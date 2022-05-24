cargo = $(env) cargo
rustup = $(env) rustup

build: build-rust copy-wasm
build-release: build-rust-release copy-wasm-release

# Linker flag "-s" for stripping (https://github.com/rust-lang/cargo/issues/3483#issuecomment-431209957)
build-rust:
	RUSTFLAGS='-C link-arg=-s' $(cargo) build --target wasm32-unknown-unknown

build-rust-release:
	RUSTFLAGS='-C link-arg=-s' $(cargo) build --release --target wasm32-unknown-unknown

copy-wasm:
	mkdir -p out
	cp "./target/wasm32-unknown-unknown/debug/{{crate_name}}.wasm" out/{{crate_name}}.wasm

copy-wasm-release:
	mkdir -p out
	cp "./target/wasm32-unknown-unknown/release/{{crate_name}}.wasm" out/{{crate_name}}.wasm

deps:
	$(rustup) target add wasm32-unknown-unknown

.PHONY : build build-release build-rust build-rust-release copy-wasm copy-wasm-release deps
