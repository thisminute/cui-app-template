Based on the https://github.com/rustwasm/wasm-pack-template.git

# Installation
This project uses wasm-build and webpack-dev-server together to serve a wasm binary compiled from a syntax called [cwl](https://github.com/thisminute/cascading-wasm-language).

To install, you will need:
1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [node/npm](https://nodejs.org/en/download/)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:
```
git clone https://github.com/thisminute/create-cwl-app.git
```

For windows users, run in the root directory:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Then:
```
cd create-cwl-app/www # npm stuff is in the www directory
wasm-pack build
npm install
npm start # opens a new browser tab in watch mode for the binary!
```

# Usage

All code goes in the `./cwl` directory in .cwl files. Example file:

```cwl
div {
   text: hello;
}
span {
   text: world;
}
```
represents
```html
<div>
   hello
</div>
<span>
   world
</span>
```
