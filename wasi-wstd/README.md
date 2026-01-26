# Use Wstd, "a WebAssembly-native Rust stdlib" library to send HTTP(s) requests from inside WASM

## Instructions for this devcontainer

Tested with Rust 1.93.0, wstd examples [v0.6.5](https://github.com/bytecodealliance/wstd/tree/v0.6.5/examples/),
Wasmtime [v41.0.0](https://github.com/bytecodealliance/wasmtime/releases/tag/v41.0.0).

### Preparation

1. Open this repo in devcontainer, e.g. using Github Codespaces.
   Type or copy/paste following commands to devcontainer's terminal.

2. Install WASI Preview 2 build target into Rust toolchain:

```sh
rustup target add wasm32-wasip2
```

### Building

1. `cd` into the folder of this example:

```sh
cd wasi-wstd
```

2. Clone the wstd repo:

```sh
git clone --depth=1 https://github.com/bytecodealliance/wstd.git
```

3. `cd` into the folder of wstd:

```sh
cd wstd
```

4. Compile all the examples:

```sh
 cargo build --release --examples -p wstd --target wasm32-wasip2
```

### Test with Wasmtime

1. Install Wasmtime:

```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

2. Run Wasmtime with HTTP plugin enabled against just-compiled HTTP Client examples:

```sh
~/.wasmtime/bin/wasmtime -S http target/wasm32-wasip2/*/examples/http_client.wasm https://httpbin.org/anything
~/.wasmtime/bin/wasmtime -S http target/wasm32-wasip2/*/examples/complex_http_client.wasm https://httpbin.org/anything
```

3. See the results in terminal.

### Finish

Perform your own experiments if desired.
