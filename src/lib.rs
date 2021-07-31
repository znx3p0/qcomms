
//! qcomms is only compatible with async std.
//! this decision was made so users aren't locked in the tokio ecosystem

mod async_comms;
pub use async_comms::{Comms, ObjComms, UdpComms, UdpObjComms};
