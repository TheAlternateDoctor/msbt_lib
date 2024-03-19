pub mod control_codes;
pub mod msbt;

// FIXME: should it perhaps be named sections?
pub mod structs;

mod error;

pub use error::{Error, Result};

// FIXME: there's probably a bunch more pub uses that could be used here

// FIXME: document

// FIXME: the entire library needs a rework on the use of unwrap and panics in general