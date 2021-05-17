

use std::io::{Read, Write, Result};

use crate::KEEPALIVE;

pub trait SyncComms: Write + Read {
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
        self.flush()?;
        Ok(())
    }

    fn receive_to_string(&mut self) -> Result<String> {
        let v = self.receive()?;
        Ok(std::string::String::from_utf8_lossy(&v).to_string())
    }

    fn receive_keepalive(&mut self, ka: &KEEPALIVE) -> Result<()> {
        let p = self.receive()?;
        if p != ka {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("keepalive failed. received {:?}", p),
            ));
        }
        Ok(())
    }

    fn send_keepalive(&mut self, ka: &KEEPALIVE) -> Result<()> {
        self.send(ka)?;
        Ok(())
    }

    fn send_handshake(&mut self, ka: &KEEPALIVE) -> Result<()> {
        self.send_keepalive(ka)?;
        self.receive_keepalive(ka)?;
        Ok(())
    }

    fn receive_handshake(&mut self, ka: &KEEPALIVE) -> Result<()> {
        self.receive_keepalive(ka)?;
        self.send_keepalive(ka)?;
        Ok(())
    }
}

impl <T: Write + Read> SyncComms for T {}