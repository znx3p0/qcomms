use std::{io::Result, marker::PhantomData, net::SocketAddr};

use async_std::{
    io::prelude::{ReadExt, WriteExt},
    net::{ToSocketAddrs, UdpSocket},
};
use futures::pin_mut;
use serde::{de::DeserializeOwned, Serialize};

macro_rules! async_t {
    (
        $v: vis
        // generics
        $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?
        $name: ident
        // generics
        $(< $( $lst:tt ),+ >)?

        $self_ident: ident

        // parameters
        ($($params: ident : $params_ty: ty),*)
        ->
        $t: ty = async $l: tt $b: block
    ) => {

        $v struct $name
        $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
        {
            $($params : $params_ty),*
        }

        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? std::future::Future for $name
        $(< $( $lst ),+ >)?
        {
            type Output = $t;
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                #[allow(unused_mut)]
                let mut $self_ident = unsafe { self.get_unchecked_mut() };
                let r_n = async $l { $b };
                pin_mut!(r_n);
                r_n.poll(cx)
            }
        }
    };
}

pub trait Comms: WriteExt + ReadExt + Unpin {
    fn receive(&mut self) -> RecvFut<'_, Self>
    where
        Self: Sized;
    fn send<'a>(&'a mut self, buf: &'a [u8]) -> SendFut<'a, Self>
    where
        Self: Sized;
}

pub trait ObjComms: WriteExt + ReadExt + Unpin {
    fn rx<O: DeserializeOwned>(&mut self) -> ObjRecvFut<'_, Self, O>
    where
        Self: Sized;
    fn tx<'a, O: Serialize>(&'a mut self, object: &'a O) -> ObjSendFut<'_, Self, O>
    where
        Self: Sized;
}

pub trait UdpComms {
    fn receive(&self) -> UdpRecvFut<'_>;
    fn send<'a>(&'a self, obj: &'a [u8]) -> UdpSendFut<'a>;

    fn send_to<'a, A: ToSocketAddrs>(&'a self, obj: &'a [u8], addr: &'a A) -> UdpSendToFut<'a, A>;
    fn receive_from<'a>(&'a self) -> UdpRecvFromFut<'a>;
}

pub trait UdpObjComms {
    fn tx<'a, T: Serialize>(&'a self, obj: &'a T) -> UdpTxFut<'a, T>;
    fn rx<T: DeserializeOwned>(&self) -> UdpRxFut<'_, T>;

    fn tx_to<'a, T: Serialize, A: ToSocketAddrs>(
        &'a self,
        obj: &'a T,
        addr: &'a A,
    ) -> UdpTxToFut<'a, T, A>;
    fn rx_from<T: DeserializeOwned>(&self) -> UdpRxFromFut<'_, T>;
}

impl UdpComms for UdpSocket {
    fn receive(&self) -> UdpRecvFut<'_> {
        UdpRecvFut { stream: self }
    }

    fn send<'a>(&'a self, buf: &'a [u8]) -> UdpSendFut<'a> {
        UdpSendFut { stream: self, buf }
    }

    fn send_to<'a, A: ToSocketAddrs>(&'a self, buf: &'a [u8], addr: &'a A) -> UdpSendToFut<'a, A> {
        UdpSendToFut {
            stream: self,
            buf,
            addr,
        }
    }

    fn receive_from<'a>(&'a self) -> UdpRecvFromFut<'a> {
        UdpRecvFromFut { stream: self }
    }
}

impl UdpObjComms for UdpSocket {
    fn tx<'a, T: Serialize>(&'a self, object: &'a T) -> UdpTxFut<'a, T> {
        UdpTxFut {
            stream: self,
            object,
        }
    }

    fn rx<T: DeserializeOwned>(&self) -> UdpRxFut<'_, T> {
        UdpRxFut {
            stream: self,
            phantom: PhantomData,
        }
    }

    fn tx_to<'a, T: Serialize, A: ToSocketAddrs>(
        &'a self,
        object: &'a T,
        addr: &'a A,
    ) -> UdpTxToFut<'a, T, A> {
        UdpTxToFut {
            stream: self,
            object,
            addr,
        }
    }

    fn rx_from<T: DeserializeOwned>(&self) -> UdpRxFromFut<'_, T> {
        UdpRxFromFut {
            stream: self,
            phantom: PhantomData,
        }
    }
}

impl<T: WriteExt + ReadExt + Unpin> Comms for T {
    fn receive(&mut self) -> RecvFut<'_, Self> {
        RecvFut { stream: self }
    }

    fn send<'a>(&'a mut self, buf: &'a [u8]) -> SendFut<'a, Self> {
        SendFut { stream: self, buf }
    }
}

impl<T: Comms + Unpin> ObjComms for T {
    fn rx<O: DeserializeOwned>(&mut self) -> ObjRecvFut<'_, Self, O>
    where
        Self: Sized,
    {
        ObjRecvFut {
            stream: self,
            phantom: PhantomData,
        }
    }

    fn tx<'a, O: Serialize>(&'a mut self, object: &'a O) -> ObjSendFut<'_, Self, O>
    where
        Self: Sized,
    {
        ObjSendFut {
            stream: self,
            object,
        }
    }
}

// udp tx fut
async_t! {
    pub <'a, S: Serialize> UdpTxFut <'a, S> this (stream: &'a UdpSocket, object: &'a S) -> Result<usize> = async move {
        let stream = &mut this.stream;
        let buf: Vec<u8> = match bincode::serialize(&this.object) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        stream.send(&buf).await
    }
}

// udp rx fut
async_t! {
    pub <'a, S: DeserializeOwned> UdpRxFut<'a, S> this
    (stream: &'a UdpSocket, phantom: PhantomData<S>) -> Result<S> =
    async move
    {
        let stream = &this.stream;
        let bytes = stream.receive().await?;
        let buf: S = match bincode::deserialize(&bytes) {
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

// udp rx from fut
async_t! {
    pub <'a, S: DeserializeOwned> UdpRxFromFut<'a, S> this
    (stream: &'a UdpSocket, phantom: PhantomData<S>) -> Result<(S, SocketAddr)> =
    async move
    {
        let stream = &mut this.stream;
        let (bytes, addr) = stream.receive_from().await?;
        let buf: S = match bincode::deserialize(&bytes) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        Ok((buf, addr))
    }
}

// udp send fut
async_t! {
    pub <'a> UdpSendFut <'a> this (stream: &'a UdpSocket, buf: &'a [u8]) -> Result<usize> = async move {
        let length: [u8; 8] = u64::to_ne_bytes(this.buf.len() as u64);
        let buf = this.buf;
        let stream = &mut this.stream;

        stream.send(&length).await?;
        stream.send(buf).await?;
        Ok(buf.len())
    }
}

// udp recv fut
async_t! {
    pub <'a> UdpRecvFut<'a> this (stream: &'a UdpSocket) -> Result<Vec<u8>> = async move {
        let mut buf = [0u8; 8];
        let stream = &mut this.stream;

        stream.recv(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        stream.recv(&mut msg).await?;
        Ok(msg)
    }
}

// udp recv from fut
async_t! {
    pub <'a> UdpRecvFromFut<'a> this (stream: &'a UdpSocket) -> Result<(Vec<u8>, SocketAddr)> = async move {
        let mut buf = [0u8; 8];
        let stream = &mut this.stream;

        stream.recv_from(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        stream.peek_from(&mut buf).await?;
        let (_, addr) = stream.recv_from(&mut msg).await?;
        Ok((msg, addr))
    }
}

// udp send to fut
async_t! {
    pub <'a, ADDR: ToSocketAddrs> UdpSendToFut<'a, ADDR> this
    (stream: &'a UdpSocket, addr: &'a ADDR, buf: &'a [u8]) -> Result<usize> =
    async move
    {
        let length: [u8; 8] = u64::to_ne_bytes(this.buf.len() as u64);
        let buf = this.buf;
        let addr = this.addr;
        let stream = &mut this.stream;

        stream.send_to(&length, addr).await?;
        stream.send_to(buf, addr).await?;
        Ok(buf.len())
    }
}

// udp tx to fut
async_t! {
    pub <'a, S: Serialize, A: ToSocketAddrs> UdpTxToFut <'a, S, A>
    this (stream: &'a UdpSocket, object: &'a S, addr: &'a A) -> Result<usize> = async move {
        let stream = &mut this.stream;
        let addr = &this.addr;

        let buf: Vec<u8> = match bincode::serialize(&this.object) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        stream.send_to(&buf, addr).await
    }
}

// send_fut
async_t! {
    pub <'a, T: WriteExt + ReadExt + Unpin> SendFut <'a, T> this (stream: &'a mut T, buf: &'a [u8]) -> Result<usize> = async move {
        let length: [u8; 8] = u64::to_ne_bytes(this.buf.len() as u64);
        let buf = this.buf;
        let stream = &mut this.stream;

        stream.write(&length).await?;
        stream.flush().await?;
        stream.write(buf).await?;
        stream.flush().await?;
        Ok(buf.len())
    }
}

// obj recv fut
async_t! {
    pub <'a, T: Comms + Unpin, S: DeserializeOwned> ObjRecvFut<'a, T, S> this
    (stream: &'a mut T, phantom: PhantomData<S>) -> Result<S> =
    async move
    {
        let stream = &mut this.stream;
        let bytes = stream.receive().await?;
        let buf: S = match bincode::deserialize(&bytes) {
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

// obj send fut
async_t! {
    pub <'a, T: Comms + Unpin, S: Serialize> ObjSendFut<'a, T, S> this
    (stream: &'a mut T, object: &'a S) -> Result<usize> =
    async move
    {
        let stream = &mut this.stream;
        let buf: Vec<u8> = match bincode::serialize(&this.object) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            }
        };
        stream.send(&buf).await
    }
}

// recv fut
async_t! {
    pub <'a, T: WriteExt + ReadExt + Unpin> RecvFut<'a, T> this (stream: &'a mut T) -> Result<Vec<u8>> = async move {
        let mut buf = [0u8; 8];
        let stream = &mut this.stream;

        stream.read_exact(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        stream.read_exact(&mut msg).await?;
        Ok(msg)
    }
}
