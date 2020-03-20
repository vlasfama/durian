# durian
Durian is a stateless VM based on  WASM for blockchain

There is a list of all tools and dependencies required for this tutorial.

### Rust
[rustup](https://github.com/rust-lang-nursery/rustup.rs#installation) is the easiest way to install Rust toolchains. Rust nightly toolchain is required since our contracts require some unstable features:

```bash
rustup install nightly-2018-11-12
```

Also, we need to install `wasm32-unknown-unknown` to compile contracts to Wasm:
```bash
rustup target add wasm32-unknown-unknown
```

### Durain

## Installation and building

   `git clone https://github.com/b00f/durian.git`

   `cd durian/cli`

* Build the durain

   `cargo build`

* Run the durian

   `cargo run ./target/debug/cli`