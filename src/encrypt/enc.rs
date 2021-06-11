use std::{
    io::{Read, Write},
    marker::PhantomData,
};

/// Defines an encryption method
pub trait Encrypt {
    /// method used to encrypt messages that are sent through a Steer
    fn encrypt<'a>(value: Vec<u8>, _key: &'a [u8]) -> Vec<u8> {
        // naive encryption
        value
    }
}
/// Defines an encryption method
pub trait Decrypt {
    /// method used to encrypt messages that are sent through a Steer
    fn decrypt<'a>(value: Vec<u8>, _key: &'a [u8]) -> Vec<u8> {
        // naive decryption
        value
    }
}

impl Encrypt for std::net::TcpStream {}
impl Decrypt for std::net::TcpStream {}

#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
impl Encrypt for tokio::net::TcpStream {}
#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
impl Encrypt for tokio::net::UnixStream {}

#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
impl Encrypt for tokio::net::UdpSocket {}
#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
impl Decrypt for tokio::net::UdpSocket {}

#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
impl Decrypt for tokio::net::TcpStream {}
#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
impl Decrypt for tokio::net::UnixStream {}

/// Synchronous version of the Steer. It is used as a wrapper for a Stream(TcpStream or UnixStream)
/// Steers are used for encryption
pub struct SyncSteer<Net: Write + Read, Enc: Encrypt + Decrypt> {
    stream: Net,
    ty: PhantomData<Enc>,
}

impl<Net: Write + Read, Enc: Encrypt + Decrypt> SyncSteer<Net, Enc> {
    // triggers warning due to features
    #[allow(unused)]
    /// Create new SyncSteer
    pub fn new(stream: Net) -> Self {
        Self {
            stream,
            ty: PhantomData::default(),
        }
    }
}

impl<Net: Write + Read, Enc: Encrypt + Decrypt> Encrypt for SyncSteer<Net, Enc> {}

impl<Net: Write + Read, Enc: Encrypt + Decrypt> std::io::Read for SyncSteer<Net, Enc> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}
impl<Net: Write + Read, Enc: Encrypt + Decrypt> std::io::Write for SyncSteer<Net, Enc> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
