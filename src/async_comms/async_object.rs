#![cfg(feature = "asynct")]
#![cfg(feature = "obj")]

use super::Comms;
use async_std::net::UdpSocket;
use async_trait::async_trait;
use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};
use std::{any::Any, io::Result};

/// ObjComms allows for serializable data structures to be sent across a stream
/// and also allows for deserializable data structures to be received from a stream
///
/// ```
/// conn.tx(1).await?;
///
/// conn.rx::<i32>().await?;
/// ```
///
#[async_trait(?Send)]
pub trait ObjComms {
    async fn tx<T: Serialize + Any>(&mut self, obj: T) -> Result<usize>;
    async fn rx<T: DeserializeOwned + Any>(&mut self) -> Result<T>;
}

#[async_trait(?Send)]
impl<X: Comms> ObjComms for X {
    async fn tx<T: Serialize>(&mut self, obj: T) -> Result<usize> {
        let buf: Vec<u8> = match serialize(&obj) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        self.send(&buf).await
    }

    async fn rx<T: DeserializeOwned>(&mut self) -> Result<T> {
        let buf = self.receive().await?;
        let buf: T = match deserialize(&buf) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        Ok(buf)
    }
}

