#![cfg(feature = "asynct")]
#![cfg(feature = "obj")]

use super::Comms;
use async_trait::async_trait;
use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};
use std::io::Result;


/// ObjComms allows for serializable data structures to be sent across a stream
/// and also allows for deserializable data structures to be received from a stream
/// 
/// ```
/// conn.tx(1).await?;
/// 
/// conn.rx::<i32>().await?;
/// ```
/// 
#[async_trait]
pub trait ObjComms: Comms {
    /// Send a serializable data structure across a stream
    /// 
    /// ``` 
    /// conn.tx(1234).await?;
    /// ```
    async fn tx<T: Serialize + Send + Sync + ?Sized>(&mut self, obj: &T) -> Result<()> {
        let buf: Vec<u8> = match serialize(obj) {
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

    /// Receive a deserializable data structure from a stream
    /// 
    /// ``` 
    /// conn.rx::<String>().await?;
    /// ```
    async fn rx<T: DeserializeOwned + Send + Sync>(&mut self) -> Result<T> {
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

impl<T: Comms> ObjComms for T {}
