#[derive(Copy,Debug,Clone)]
pub enum Endian {
    BigEndian,
    LittleEndtian,
}

pub(crate) fn system_endian() -> Endian {
    if cfg!(tarread_endian = "big") {
        Endian::BigEndian
    } else {
        Endian::LittleEndtian
    }
}