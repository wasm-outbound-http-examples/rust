# Make HTTP requests from inside WASM in Rust

This devcontainer is configured to provide you a latest stable version of Rust toolset.

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/wasm-outbound-http-examples/rust)

### Browser / JS runtime Examples

<table>
<tr>
<th>#</th>
<th>Example</th>
<th>Description</th>
<th>Browser demo</th>
</tr>
<tr>
<td>1</td>
<td>

[quad-net](browser-quad-net/README.md)

</td>
<td>

Use `quad-net` plugin for Miniquad to send HTTP requests from web browser.

</td>
<td>

[Demo](https://wasm-outbound-http-examples.github.io/rust/quad-net/)

</td>
</tr>
</table>

### WASI / Standalone / Server-side Examples

<table>
<tr>
<th>#</th>
<th>Example</th>
<th>Description</th>
<th>Compatibility</th>
</tr>
<tr>
<td>1</td>
<td>

[waki](wasi-waki/README.md)

</td>
<td>

Use `waki` WASI HTTP library to send HTTP requests from Wasmtime.

</td>
<td>

WASI Preview 2 / WASI 0.2

</td>
</tr>
<tr>
<td>2</td>
<td>

[`wstd::http::Client`](wasi-wstd/README.md)

</td>
<td>

Use `wstd` WASI "stdlib" to send HTTP requests from Wasmtime.

</td>
<td>

WASI Preview 2 / WASI 0.2

</td>
</tr>
<tr>
<td>3</td>
<td>

[`wstd::net::TcpStream`](wasip2-sockets-wstd/README.md)

</td>
<td>

Use "raw" wasip2 sockets and `wstd`'s `TcpStream` to send HTTP requests from Wasmtime.

</td>
<td>

WASI Preview 2 / WASI 0.2

</td>
</tr>
</table>

<sub>Created for (wannabe-awesome) [list](https://github.com/vasilev/HTTP-request-from-inside-WASM)</sub>
