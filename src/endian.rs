use std::fmt::Display;

#[derive(Copy,Debug,Clone,PartialEq)]
pub enum Endian {
    BigEndian,
    LittleEndian,
}

impl Endian {
    pub fn as_str(&self) -> &str {
        match &self {
            Endian::BigEndian => {
                "Big Endian"
            },
            Endian::LittleEndian => {
                "Little Endian"
            },
        }
    }
}

impl Display for Endian {
    
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"{}",&self.as_str())
    }
}

pub(crate) fn system_endian() -> Endian {
    if cfg!(tarread_endian = "big") {
        Endian::BigEndian
    } else {
        Endian::LittleEndian
    }
}

