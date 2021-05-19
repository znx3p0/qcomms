use std::io::{Read, Result, Write};

use crate::{
    encrypt::{Decrypt, Encrypt},
    KEEPALIVE,
};

/// The synchronous version of Comms
pub trait SyncComms: Write + Read + Encrypt + Decrypt {
    /// Receives a message from a stream
    fn receive(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        self.read_exact(&mut msg)?;
        Ok(msg)
    }
    
    /// Sends a message to a stream
    fn send(&mut self, buf: &[u8]) -> Result<()> {
        let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
        self.write(&length)?;
        self.flush()?;
        self.write(buf)?;
        self.flush()?;
        Ok(())
    }

    /// Receive a message and turn it into a String
    fn receive_to_string(&mut self) -> Result<String> {
        let v = self.receive()?;
        Ok(std::string::String::from_utf8_lossy(&v).to_string())
    }

    /// Receives a keepalive message
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

    /// Sends a keepalive message
    fn send_keepalive(&mut self) -> Result<()> {
        self.send(KEEPALIVE)?;
        Ok(())
    }

    /// Sends a handshake message.
    /// This sends a keepalive message, and then receives a keepalive message
    fn send_handshake(&mut self) -> Result<()> {
        self.send_keepalive()?;
        self.receive_keepalive()?;
        Ok(())
    }
    
    /// Receives a handshake message.
    /// This receives a keepalive message and then sends a keepalive message
    fn receive_handshake(&mut self) -> Result<()> {
        self.receive_keepalive()?;
        self.send_keepalive()?;
        Ok(())
    }
}

impl<T: Write + Read + Encrypt + Decrypt> SyncComms for T {}
