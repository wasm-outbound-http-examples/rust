// Partially based on https://github.com/bytecodealliance/wstd/blob/v0.6.5/examples/tcp_stream_client.rs
use wstd::io::{self, AsyncRead, AsyncWrite};
use wstd::net::TcpStream;

#[wstd::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("httpbin.org:80").await?;
    stream
        .write_all(b"GET /anything HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n")
        .await?;

    let mut resp_buffer = Vec::new();
    stream.read_to_end(&mut resp_buffer).await?;

    let response = String::from_utf8(resp_buffer).expect("Invalid utf bytes");
    println!("resp: {}", response);

    Ok(())
}
