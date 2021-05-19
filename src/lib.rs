#![deny(missing_docs)]

//!
//! qcomms has a set of helper traits and two data structures which help
//! send data around when working with streams
//!

/// Keepalive message used for the Keepalive methods
pub const KEEPALIVE: &[u8; 4] = &[74, 197, 182, 85];

mod async_comms;

/// Has encryption traits and the Steer and SyncSteer helper structures
pub mod encrypt;
mod sync_comms;

#[cfg(feature = "asynct")]
pub use async_comms::comms::*;

#[cfg(feature = "asynct")]
#[cfg(feature = "obj")]
pub use async_comms::async_object::*;

/// Has the synchronous version of Comms and ObjComms
pub mod sync {
    pub use crate::sync_comms::*;
}
