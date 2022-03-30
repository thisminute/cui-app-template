## About

This repository is a Rust project that serves as a barebones template for compiling [CUI](https://github.com/thisminute/cascading-ui) code into a deployable package. If you are just getting started with CUI, the [cui-tools repository](https://github.com/thisminute/cui-tools) has more information in the README and should be a better place to start.

Remember to use `wasm-pack build --target web` and not `cargo build`!

## Dependencies
To build CUI, you will need:

1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:

```
git clone https://github.com/thisminute/cui-app-template.git
```

For windows users, run in the root directory:

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

## Usage

1. Modify the source code in `src/lib.rs`
1. Run `wasm-pack build --target web`

This will create a `pkg` directory with an `index.html`. The `pkg` directory can be served to deploy the application! The [cui-devtools repository](https://github.com/thisminute/cui-devtools) is recommended if you want to serve the application locally for testing purposes.
