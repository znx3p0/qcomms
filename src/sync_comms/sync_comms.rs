use std::io::{Read, Result, Write};

use crate::{
    KEEPALIVE,
};

/// The Comms trait has various helper methods to work with streams
pub trait SyncComms {
    /// Receives a vector of bytes from a stream representing a message
    fn receive(&mut self) -> Result<Vec<u8>>;
    /// Sends a slice of bytes representing a message
    fn send(&mut self, buf: &[u8]) -> Result<()>;
    /// Receives a keepalive message
    fn receive_keepalive(&mut self) -> Result<()>;
    /// Sends a keepalive message
    fn send_keepalive(&mut self) -> Result<()>;
}

impl<T: Write + Read> SyncComms for T {
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
        self.flush()
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
        self.send(KEEPALIVE)
    }
}
