
#![cfg(feature = "obj")]

use bincode::{deserialize, serialize};
use serde::{Serialize, de::DeserializeOwned};
use std::io::Result;

use super::sync_comms::SyncComms;






pub trait SyncObjComms: SyncComms {
    fn tx<T: Serialize + Send + Sync + ?Sized>(&mut self, obj: &T) -> Result<()> {
        let buf = match serialize(obj) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        self.send(&buf)?;

        Ok(())
    }

    fn rx<T: DeserializeOwned + Send + Sync>(&mut self) -> Result<T> {
        let d = self.receive()?;
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

impl<T: SyncComms> SyncObjComms for T {}






