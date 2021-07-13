use std::any::Any;

use async_channel::{unbounded, Receiver, Sender};
use serde::{Serialize, de::DeserializeOwned};
use std::io::*;
use async_trait::async_trait;

use crate::ObjComms;

pub struct Stream {
    rx: Receiver<Box<dyn Any + Send>>,
    tx: Sender<Box<dyn Any + Send>>,
}

impl Stream {
    pub fn new() -> (Self, Self) {
        let (tx, rx) = unbounded();
        let (tx1, rx1) = unbounded();
        (Self { rx, tx: tx1 }, Self { rx: rx1, tx })
    }
    pub async fn tx<T: Send + Any>(&self, obj: T) -> Result<()> {
        match self.tx.try_send(Box::new(obj)) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                ErrorKind::BrokenPipe,
                e.to_string(),
            )),
        }
    }

    pub async fn rx<T: Any>(&mut self) -> Result<T> {
        match self.rx.recv().await {
            Ok(obj) => match obj.downcast::<T>() {
                Ok(o) => Ok(*o),
                Err(e) => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("received object type id {:?}", e.type_id()),
                )),
            },
            Err(e) => Err(Error::new(
                ErrorKind::BrokenPipe,
                e.to_string(),
            )),
        }
    }
}

#[async_trait(?Send)]
impl ObjComms for Stream {
    async fn tx<T: Serialize + Any>(&mut self, obj: T) -> Result<usize> {
        self.tx(obj).await
    }

    async fn rx<T: DeserializeOwned + Any>(&mut self) -> Result<T> {
        self.rx().await
    }
}
