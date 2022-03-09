# convoluted-mirror

![](images/convolution_01.png)

Experimental mirror built with webAssembly, Rust and javascript

## Commands

- `yarn` to install dependencies
- `yarn build` to create dist folder
- `yarn wasm` to build the WASM file!
- `yarn start` to serve index.html
- `yarn format` to format your changes (needs ftm installed - run `cargo fmt`)

## How to start

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install wasm-pack: https://rustwasm.github.io/wasm-pack/installer/
3. Install cargo-generate: `cargo install cargo-generate`
4. Clone repo and `cd` into `convoluted-mirror` folder
5. run the following commands:
   - `yarn`
   - `yarn wasm`
   - `yarn start`

## Usefull vscode extensions

- `bungcip.better-toml` for the cargo generated .toml files
- `rust-lang.rust` for rust syntax highlighting
- `dtsvet.vscode-wasm` for syntax highlighting and wasm binary view
