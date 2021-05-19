#![cfg(feature = "obj")]

use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};
use std::io::Result;

use super::sync_comms::SyncComms;

/// Allows for data structures to be sent and received from synchronous streams
pub trait SyncObjComms: SyncComms {
    /// Send a serializable data structure across a stream.
    ///
    /// ```
    /// #[derive(Serialize)]
    /// struct Message(u32);
    ///
    /// conn.tx(Message(2));
    /// ```
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

    /// Receive a deserializable data structure from a stream.
    ///
    /// ```
    /// #[derive(Deserialize)]
    /// struct Message(u32);
    ///
    /// conn.rx::<Message>();
    /// ```
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
