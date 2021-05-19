#![cfg(feature = "asynct")]

use async_trait::async_trait;
use std::io::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::KEEPALIVE;

/// The Comms trait has various helper methods to work with streams
#[async_trait]
pub trait Comms: AsyncWrite + AsyncRead + AsyncWriteExt + Unpin {
    /// Receives a vector of bytes from a stream representing a message
    async fn receive(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        self.read_exact(&mut msg).await?;
        Ok(msg)
    }

    /// Sends a slice of bytes representing a message
    async fn send(&mut self, buf: &[u8]) -> Result<()> {
        let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
        self.write(&length).await?;
        self.flush().await?;
        self.write(buf).await?;
        self.flush().await?;
        Ok(())
    }

    /// Receives a message and turns it into a string
    async fn receive_to_string(&mut self) -> Result<String> {
        let v = self.receive().await?;
        Ok(std::string::String::from_utf8_lossy(&v).to_string())
    }

    /// Receives a keepalive message
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

    /// Sends a keepalive message
    async fn send_keepalive(&mut self) -> Result<()> {
        self.send(KEEPALIVE).await?;
        Ok(())
    }

    /// Sends a handshake message.
    /// This sends a keepalive message, and then receives a keepalive message
    async fn send_handshake(&mut self) -> Result<()> {
        self.send_keepalive().await?;
        self.receive_keepalive().await?;
        Ok(())
    }

    /// Receives a handshake message.
    /// This receives a keepalive message and then sends a keepalive message
    async fn receive_handshake(&mut self) -> Result<()> {
        self.receive_keepalive().await?;
        self.send_keepalive().await?;
        Ok(())
    }
}

impl<T: AsyncWrite + AsyncRead + AsyncWriteExt + Unpin> Comms for T {}
