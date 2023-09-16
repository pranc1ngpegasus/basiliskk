use crate::adapter::codec::SkkCodecImpl;
use crate::domain::error::BasiliskkErr;
use crate::domain::listener::Listener;
use async_trait::async_trait;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::FramedRead;
use tracing::{debug, error};

pub struct ListenerImpl {
    addr: SocketAddr,
}

impl ListenerImpl {
    pub fn new(port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
        Self { addr }
    }
}

#[async_trait]
impl Listener for ListenerImpl {
    async fn listen(&self) -> Result<(), BasiliskkErr> {
        let listener = TcpListener::bind(self.addr)
            .await
            .map_err(|e| BasiliskkErr {
                desc: format!("failed to bind addr; {}", e),
            })?;

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    debug!("Request accepted from: {}", addr);

                    let mut frame_reader = FramedRead::new(socket, SkkCodecImpl::new());
                    while let Some(frame) = frame_reader.next().await {
                        match frame {
                            Ok(data) => debug!("Received data: {:?}", data),
                            Err(e) => error!("failed to read data; {}", e),
                        }
                    }
                }
                Err(e) => {
                    error!("failed to accept socket; {}", e);
                }
            }
        }
    }
}
