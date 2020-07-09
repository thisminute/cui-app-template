Based on the https://github.com/rustwasm/wasm-pack-template.git

This project uses wasm-build and webpack-dev-server together to serve a wasm binary compiled from a syntax called (CWF)[https://github.com/thisminute/cascading-wasm-framework].

Then:
```
git clone https://github.com/thisminute/create-cwf-app.git
```

For windows users, run in the root directory:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Then:
```
cd create-cwf-app/www    # npm stuff is in the www directory
npm run clean # will run wasm-pack and npm install
npm start     # opens a new browser tab in watch mode for the binary!
```

Once started, try editing src/lib.rs. Example:

```cwf
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

Try editing the tag types or content of the elements, or adding more!

# Understanding the Code

## Procedural Macro
`./src/lib.rs` is the entry point for the procedural macro that generates Rust code from Cwf file input.

`./src/tokens.rs` is the definition file for the tokens that the language consists of. Rule and List are the most important tokens.

## `create-cwf-app`
`./create-cwf-app` creates a node/webpack server that is used to test and deploy the language. See its README for more!
