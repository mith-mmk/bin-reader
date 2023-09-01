//! Crate bin_rs is a binary manipulate crate.
//!

pub mod reader;
#[cfg(feature="async")]
pub mod async_reader;
#[cfg(feature="util")]
pub mod io;
pub mod endian;
pub use endian::*;
#[cfg(test)]
pub mod test;
