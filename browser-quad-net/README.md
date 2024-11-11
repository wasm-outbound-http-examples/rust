# Use quad-net plugin for Miniquad to send HTTP(s) requests from inside WASM

## Instructions for this devcontainer

This demo uses (slightly patched) code from official quad-net [example](https://github.com/not-fl3/quad-net/blob/31796d4ed5c9cce57c7702d18b6140d5ffacfccf/examples/http_request.rs).

Tested with Rust 1.82.0, quad-net [commit 31796d4ed](https://github.com/not-fl3/quad-net/tree/31796d4ed5c9cce57c7702d18b6140d5ffacfccf).

### Preparation

1. Open this repo in devcontainer, e.g. using Github Codespaces.
   Type or copy/paste following commands to devcontainer's terminal.

### Building

1. `cd` into the folder of this example:

```sh
cd browser-quad-net
```

2. Clone the quad-net repo:

```sh
git clone --depth=1 https://github.com/not-fl3/quad-net.git
```

3. `cd` into the cloned repo:

```sh
cd quad-net
```

4. Patch the provided example to make a HTTP request to remote website:

```sh
sed -i.bak 's|http://127.0.0.1:4000/|https://httpbin.org/anything|' examples/http_request.rs
sed -i.bak 's|client.wasm|http_request.wasm|' examples/web/index.html
```

5. Compile the example:

```sh
cargo build --example http_request --target wasm32-unknown-unknown --release
```

6. Copy the built WASM file to example's `web` folder:

```sh
cp target/wasm32-unknown-unknown/release/examples/http_request.wasm examples/web/
```

7. Copy the demo's file into current folder:

```sh
cp ../index.html ./
```

### Test with browser

1. Run simple HTTP server to temporarily publish project to Web:

```sh
python3 -m http.server
```

Codespace will show you "Open in Browser" button. Just click that button or
obtain web address from "Forwarded Ports" tab.

2. As HTML, JS files, and a 450k-sized WASM file are loaded into browser, press a "HTTP Request" button and refer to browser developer console
   to see the results.

### Finish

Perform your own experiments if desired.
