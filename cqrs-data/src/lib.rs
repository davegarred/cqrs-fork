//! This is documented.

extern crate cqrs;
extern crate hashbrown;
extern crate parking_lot;
extern crate void;

pub mod memory;
pub mod trivial;

mod event;
mod snapshot;

mod entity;
mod types;

pub use event::{EventSink, EventSource};
pub use snapshot::{SnapshotSink, SnapshotSource};
pub use entity::Entity;
pub use types::Since;