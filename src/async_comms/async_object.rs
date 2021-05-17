

#![cfg(feature = "asynct")]
#![cfg(feature = "obj")]



use async_trait::async_trait;
use super::Comms;
use std::io::Result;
use serde::{Serialize, de::DeserializeOwned};
use bincode::{serialize, deserialize};

#[async_trait]
pub trait ObjComms: Comms {
    async fn tx<T: Serialize + Send + Sync + ?Sized>(&mut self, obj: &T) -> Result<()> {
        let buf = match serialize(obj) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        self.send(&buf).await?;

        Ok(())
    }

    async fn rx<T: DeserializeOwned + Send + Sync>(&mut self) -> Result<T> {
        let d = self.receive().await?;
        let d: T = match deserialize(&d) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        Ok(d)
    }
}

impl<T: Comms> ObjComms for T {}
