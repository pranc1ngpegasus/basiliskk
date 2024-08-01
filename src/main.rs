use anyhow::Context;
use encoding_rs::EUC_JP;
use std::io::{BufRead, BufReader, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 1179);
    let listener = TcpListener::bind(addr).context("failed to bind socket: {e}")?;

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                let mut reader = BufReader::new(&stream);

                let header = {
                    let mut data: [u8; 1] = [0; 1];
                    reader
                        .read_exact(&mut data)
                        .context("failed to read header: {e}")?;

                    match data[0] {
                        b'0' => "disconnect",
                        b'1' => "conversion",
                        b'2' => "version",
                        b'3' => "hostinfo",
                        b'4' => "completion",
                        _ => panic!("invalid header: {data:?}"),
                    }
                };

                let body = {
                    let mut data = Vec::new();
                    let _ = reader
                        .read_until(b' ', &mut data)
                        .context("failed to read body: {e}")?;

                    match EUC_JP.decode(data.trim_ascii_end()) {
                        (decoded, _, false) => decoded.to_string(),
                        (_, _, true) => {
                            panic!("failed to decode: {data:?}");
                        }
                    }
                };

                println!("header: {header:?}, body: {body:?}");
            }
            Err(e) => {
                eprintln!("failed to accept socket: {e}");
            }
        }
    }
}
