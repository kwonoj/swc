
//https://github.com/vmx/wasm-multi-value-reverse-polyfill/blob/master/src/main.rs

//https://github.com/rust-lang/rust/issues/73755
/*
In your build.rs file, you do something like this, but either wrap it with a command, or run a shell script through a command.
Building to an alternate target directory solves the cargo locking issue
cargo build -p mywasm \
  --target=wasm32-unknown-unknown \
  --target-dir=alt-target
  --release

wasm-bindgen --target=web \
  --out-dir=final-out-dir \
  alt-target/wasm32-unknown-unknown/release/mywasm.was
*/