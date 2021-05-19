#![cfg(feature = "asynct")]

use std::marker::PhantomData;

use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    encrypt::{Decrypt, Encrypt},
    Comms,
};
use std::io::Result;

/// Wrapper type around a stream that allows for encryption
pub struct Steer<'a, Stream: Comms + Send + Sync, Encryptor> {
    pub(crate) stream: Stream,
    pub(crate) key: &'a [u8],
    ty: PhantomData<Encryptor>,
}

impl<'a, Stream: Comms + Send + Sync, Encryptor: Encrypt + Decrypt> Steer<'a, Stream, Encryptor> {
    /// Create a new steer
    pub fn new(stream: Stream, key: &'a [u8]) -> Self {
        Self {
            stream,
            key,
            ty: PhantomData::default(),
        }
    }

    /// Receive a deserializable data structure from a stream
    /// ```
    /// #[derive(Deserialize)]
    /// struct Message(u32);
    ///
    /// steer.rx::<Message>().await?;
    /// ```
    pub async fn rx<T: DeserializeOwned + Send + Sync>(&mut self) -> Result<T> {
        let buf = self.stream.receive().await?;
        let buf = Encryptor::encrypt(buf, self.key);
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

    /// Send a serializable data structure across a stream
    /// ```
    /// #[derive(Serialize)]
    /// struct Message(u32);
    ///
    /// steer.tx(Message(2)).await?;
    /// ```
    pub async fn tx<T: Serialize + Send + Sync + ?Sized>(&mut self, obj: &T) -> Result<()> {
        let buf: Vec<u8> = match serialize(obj) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        let buf = Encryptor::encrypt(buf, self.key);
        self.stream.send(&buf).await
    }
    /// Sets the encryption key
    pub fn set_key<'b: 'a>(&mut self, k: &'b [u8]) {
        self.key = k;
    }
}
