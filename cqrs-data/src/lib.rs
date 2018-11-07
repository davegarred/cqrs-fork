#! This is documented.

extern crate cqrs;
extern crate hashbrown;
extern crate parking_lot;
extern crate void;

pub mod memory;
pub mod trivial;

mod event;
mod state;

mod entity;
mod types;

pub use event::{EventSink, EventSource};
pub use state::{SnapshotSink, SnapshotSource};
pub use entity::Entity;
pub use types::Since;