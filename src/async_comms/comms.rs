#![cfg(feature = "asynct")]

use async_trait::async_trait;
use std::io::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::KEEPALIVE;

#[async_trait]
/// The Comms trait has various helper methods to work with streams
pub trait Comms {
    /// Receives a vector of bytes from a stream representing a message
    async fn receive(&mut self) -> Result<Vec<u8>>;
    /// Sends a slice of bytes representing a message
    async fn send(&mut self, buf: &[u8]) -> Result<()>;
    /// Receives a keepalive message
    async fn receive_keepalive(&mut self) -> Result<()>;
    /// Sends a keepalive message
    async fn send_keepalive(&mut self) -> Result<()>;
}

#[async_trait]
impl<T: AsyncWrite + AsyncRead + AsyncWriteExt + Unpin + Send> Comms for T {
    async fn receive(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        self.read_exact(&mut msg).await?;
        Ok(msg)
    }

    async fn send(&mut self, buf: &[u8]) -> Result<()> {
        let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
        self.write(&length).await?;
        self.flush().await?;
        self.write(buf).await?;
        self.flush().await
    }

    async fn receive_keepalive(&mut self) -> Result<()> {
        let p = self.receive().await?;
        if p != KEEPALIVE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("keepalive failed. received {:?}", p),
            ));
        }
        Ok(())
    }

    async fn send_keepalive(&mut self) -> Result<()> {
        self.send(KEEPALIVE).await?;
        Ok(())
    }
}
