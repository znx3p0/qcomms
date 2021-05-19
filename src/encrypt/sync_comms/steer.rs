#![cfg(feature = "obj")]

use std::marker::PhantomData;

use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};

use std::io::Result;

use crate::{
    encrypt::{Decrypt, Encrypt},
    sync::SyncComms,
};

/// SyncSteer is a wrapper around a stream which implements encryption
pub struct SyncSteer<'a, Stream: SyncComms + Send + Sync, Encryptor> {
    pub(crate) stream: Stream,
    pub(crate) key: &'a [u8],
    ty: PhantomData<Encryptor>,
}

impl<'a, Stream: SyncComms + Send + Sync, Encryptor: Encrypt + Decrypt>
    SyncSteer<'a, Stream, Encryptor>
{
    /// Create a new SyncSteer
    pub fn new(stream: Stream, key: &'a [u8]) -> Self {
        Self {
            stream,
            key,
            ty: PhantomData::default(),
        }
    }

    /// Receive a data structure which implements Deserialize
    pub fn rx<T: DeserializeOwned + Send + Sync>(&mut self) -> Result<T> {
        let buf = self.stream.receive()?;
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

    /// Send a data structure which implements Serialize
    /// ```
    /// #[derive(Serialize, Deserialize)]
    /// struct SomeMessage(u32)
    /// 
    /// conn.tx(SomeMessage(2));
    /// ```
    pub fn tx<T: Serialize + Send + Sync + ?Sized>(&mut self, obj: &T) -> Result<()> {
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
        self.stream.send(&buf)
    }
    /// changes the key used for encryption
    pub fn set_key<'b: 'a>(&mut self, k: &'b [u8]) {
        self.key = k;
    }
}
