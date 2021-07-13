#![cfg(feature = "asynct")]

use async_trait::async_trait;
use std::io::Result;
use async_std::{io::prelude::{
        WriteExt, Write, Read, ReadExt
    }, net::{TcpStream, ToSocketAddrs, UdpSocket}};
use tokio::net::TcpStream as TokioTcpStream;
use tokio::net::UnixStream;
use tokio::io::*;


use crate::KEEPALIVE;

macro_rules! implement {
    ($ty: ty) => {
        #[async_trait(?Send)]
        impl Comms for $ty {
            async fn receive(&mut self) -> Result<Vec<u8>> {
                let mut buf = [0u8; 8];
                self.read_exact(&mut buf).await?;
                let length = u64::from_ne_bytes(buf);
                let mut msg = vec![0u8; length as usize];
                self.read_exact(&mut msg).await?;
                Ok(msg)
            }
            async fn send(&mut self, buf: &[u8]) -> Result<usize> {
                let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
                self.write(&length).await?;
                self.flush().await?;
                self.write(buf).await?;
                self.flush().await?;
                Ok(buf.len())
            }
            async fn receive_keepalive(&mut self) -> Result<()> {
                let kpalive = self.receive().await?;
                if kpalive != KEEPALIVE {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("keepalive failed. received {:?}", kpalive),
                    ));
                }
                Ok(())
            }
            async fn send_keepalive(&mut self) -> Result<()> {
                self.send(KEEPALIVE).await?;
                Ok(())
            }
        }
    };
}
macro_rules! implement_all {
    ($($ty: ty,)*) => {
        $(
            implement!(
                $ty
            );
        )*
    };
}

implement_all! {
    TcpStream,
    TokioTcpStream,
    UnixStream,
}

#[async_trait(?Send)]
/// The Comms trait has various helper methods to work with streams
pub trait Comms {
    /// Receives a vector of bytes from a stream representing a message
    async fn receive(&mut self) -> Result<Vec<u8>>;
    /// Sends a slice of bytes representing a message
    async fn send(&mut self, buf: &[u8]) -> Result<usize>;
    /// Receives a keepalive message
    async fn receive_keepalive(&mut self) -> Result<()>;
    /// Sends a keepalive message
    async fn send_keepalive(&mut self) -> Result<()>;
}

#[async_trait(?Send)]
/// The Comms trait has various helper methods to work with streams
pub trait UdpComms: Send + Sync {
    /// Receives a vector of bytes from a stream representing a message
    async fn receive(&self) -> Result<Vec<u8>>;
    /// Sends a slice of bytes representing a message
    async fn send(&self, buf: &[u8]) -> Result<()>;
    /// Sends a slice of bytes representing a message
    async fn send_to<A>(&self, buf: &[u8], addrs: A) -> Result<usize>
    where
        A: ToSocketAddrs;
    /// Receives a keepalive message
    async fn receive_keepalive(&self) -> Result<()>;
    /// Sends a keepalive message
    async fn send_keepalive(&self) -> Result<()>;
}

#[async_trait(?Send)]
impl Comms for UdpSocket {
    async fn receive(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 8];
        self.recv(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        self.recv(&mut msg).await?;
        Ok(msg)
    }

    async fn send(&mut self, buf: &[u8]) -> Result<usize> {
        let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
        self.send(&length).await?;
        self.send(buf).await?;
        Ok(buf.len())
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
