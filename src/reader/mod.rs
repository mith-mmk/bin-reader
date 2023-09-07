//! A reader default uses system endianness
//! If you will use another endianness,use set_endian.
//! 0.0.9 enable Stream Reader is default but not enable wasm


mod binary;
mod bytes;
mod stream;
pub use binary::BinaryReader as BinaryReader;
pub use bytes::BytesReader as BytesReader;
pub use stream::StreamReader as StreamReader;

#[cfg(feature="codec")]
use encoding_rs::*;

#[cfg(feature="codec")]
pub enum CodeType {
    Ascii,
    Big5,
    EucJp,
    EucKr,
    Gb18030,
    Jis,
    ShiftJis,
    Utf16Be,
    Utf16le,
    Utf8,
}