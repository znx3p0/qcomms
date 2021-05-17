
pub type KEEPALIVE = [u8; 4];

mod async_comms;
mod sync_comms;

#[cfg(feature = "asynct")]
pub use async_comms::comms::*;

#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
pub use async_comms::async_object::*;

pub use sync_comms::*;

