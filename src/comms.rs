use std::io::{Read, Write};

const KEEPALIVE: &[u8; 4] = &[19, 233, 92, 175];

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use std::io::Result;
#[async_trait]
pub trait Comms: AsyncWrite + AsyncRead + AsyncWriteExt + Unpin {
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
        self.flush().await?;
        Ok(())
    }

    async fn receive_to_string(&mut self) -> Result<String> {
        let v = self.receive().await?;
        Ok(std::string::String::from_utf8_lossy(&v).to_string())
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

    async fn send_handshake(&mut self) -> Result<()> {
        self.send_keepalive().await?;
        self.receive_keepalive().await?;
        Ok(())
    }

    async fn receive_handshake(&mut self) -> Result<()> {
        self.receive_keepalive().await?;
        self.send_keepalive().await?;
        Ok(())
    }
}

pub trait SyncComms: Write + Read {
    fn receive(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        self.read_exact(&mut msg)?;
        Ok(msg)
    }

    fn send(&mut self, buf: &[u8]) -> Result<()> {
        let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
        self.write(&length)?;
        self.flush()?;
        self.write(buf)?;
        self.flush()?;
        Ok(())
    }

    fn receive_to_string(&mut self) -> Result<String> {
        let v = self.receive()?;
        Ok(std::string::String::from_utf8_lossy(&v).to_string())
    }

    fn receive_keepalive(&mut self) -> Result<()> {
        let p = self.receive()?;
        if p != KEEPALIVE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("keepalive failed. received {:?}", p),
            ));
        }
        Ok(())
    }

    fn send_keepalive(&mut self) -> Result<()> {
        self.send(KEEPALIVE)?;
        Ok(())
    }

    fn send_handshake(&mut self) -> Result<()> {
        self.send_keepalive()?;
        self.receive_keepalive()?;
        Ok(())
    }

    fn receive_handshake(&mut self) -> Result<()> {
        self.receive_keepalive()?;
        self.send_keepalive()?;
        Ok(())
    }
}

impl<T: AsyncRead + AsyncWrite + Unpin> Comms for T {}
impl<T: Write + Read> SyncComms for T {}
