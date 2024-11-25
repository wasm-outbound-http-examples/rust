# Use Waki HTTP library to send HTTP(s) requests from inside WASM

## Instructions for this devcontainer

This demo is partially based on instruction for official `waki` 
[example](https://github.com/wacker-dev/waki/blob/v0.5.0/examples/client/README.md#http-client).

Tested with Rust 1.83.0, waki examples [v0.5.0](https://github.com/wacker-dev/waki/tree/v0.5.0/examples),
Wasmtime [v27.0.0](https://github.com/bytecodealliance/wasmtime/releases/tag/v27.0.0).

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
cd wasi-waki
```

2. Clone the waki repo:

```sh
git clone --depth=1 https://github.com/wacker-dev/waki.git
```

3. `cd` into the folder of HTTP client example:

```sh
cd waki/examples/client
```

4. Compile the example:

```sh
cargo build --release
```

### Test with Wasmtime

1. Install Wasmtime:

```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

2. Run Wasmtime with HTTP plugin enabled against just-compiled WASM file:

```sh
~/.wasmtime/bin/wasmtime -S http target/wasm32-wasip2/*/http-client.wasm
```

3. See the results in terminal.

### Finish

Perform your own experiments if desired.
