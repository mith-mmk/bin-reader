use std::io::Error;
use std::io::ErrorKind;
use crate::Endian;

#[cfg(feature="codec")]
use encoding_rs::*;

#[cfg(feature="stream")]
use std::io::BufRead;

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

pub trait BinaryReader {
    fn read_byte(&mut self) -> Result<u8,Error>;
    fn read_u8(&mut self) -> Result<u8,Error>;
//    fn read_bytes(&mut self,len: usize) -> Result<&[u8],Error>;
    fn read_bytes_as_vec(&mut self,len: usize) -> Result<Vec<u8>,Error>;

    /// read_bytes_no_move does not move offset after read_bytes.
    ///
    /// Ex)
    /// 
    /// ```
    /// let buffer = b"Hello World!";
    /// let mut reader = BytesReader::new(buffer);
    /// let buffer1 = reader.read_bytes_no_move(4)?;
    /// assert_eq!(buffer1,b"Hell");
    /// let buffer1 = reader.read_bytes_as_vec(4)?;
    /// assert_eq!(buffer1,b"Hell");
    /// let buffer1 = reader.read_bytes_as_vec(4)?;
    /// assert_eq!(buffer1,b"o Wor");
    /// ```
    /// 
    fn read_bytes_no_move(&mut self,len: usize) -> Result<Vec<u8>,Error>;

    fn read_u16(&mut self) -> Result<u16,Error>;
    fn read_u32(&mut self) -> Result<u32,Error>;
    fn read_u64(&mut self) -> Result<u64,Error>;
    fn read_u128(&mut self) -> Result<u128,Error>;
    fn read_i8(&mut self) -> Result<i8,Error>;
    fn read_i16(&mut self) -> Result<i16,Error>;
    fn read_i32(&mut self) -> Result<i32,Error>;
    fn read_i64(&mut self) -> Result<i64,Error>;
    fn read_i128(&mut self) -> Result<i128,Error>;

    fn read_f32(&mut self) -> Result<f32,Error>;
    fn read_f64(&mut self) -> Result<f64,Error>;

    fn read_u16_be(&mut self) -> Result<u16,Error>;
    fn read_u32_be(&mut self) -> Result<u32,Error>;
    fn read_u64_be(&mut self) -> Result<u64,Error>;
    fn read_u128_be(&mut self) -> Result<u128,Error>;
    fn read_i16_be(&mut self) -> Result<i16,Error>;
    fn read_i32_be(&mut self) -> Result<i32,Error>;
    fn read_i64_be(&mut self) -> Result<i64,Error>;
    fn read_i128_be(&mut self) -> Result<i128,Error>;

    fn read_f32_be(&mut self) -> Result<f32,Error>;
    fn read_f64_be(&mut self) -> Result<f64,Error>;

    fn read_u16_le(&mut self) -> Result<u16,Error>;
    fn read_u32_le(&mut self) -> Result<u32,Error>;
    fn read_u64_le(&mut self) -> Result<u64,Error>;
    fn read_u128_le(&mut self) -> Result<u128,Error>;
    fn read_i16_le(&mut self) -> Result<i16,Error>;
    fn read_i32_le(&mut self) -> Result<i32,Error>;
    fn read_i64_le(&mut self) -> Result<i64,Error>;
    fn read_i128_le(&mut self) -> Result<i128,Error>;

    fn read_f32_le(&mut self) -> Result<f32,Error>;
    fn read_f64_le(&mut self) -> Result<f64,Error>;

    /// read_ascii_string for C like ascii string.This function finishes find end marker 0x00.
    /// If reader read until \0, but skip size byte.
    /// 
    /// Ex)
    /// 
    /// ```
    /// let buffer = b"Hello World!\01234";
    /// let mut reader = BytesReader::new(buffer);
    /// let r = reader.read_ascii_string("Hello World!\01234".len())?;  // after \0 is trim
    /// assert_eq!(r ,"Hello World!");
    /// ```
    /// 
    fn read_ascii_string(&mut self,size:usize) -> Result<String,Error>;

    fn read_utf8_string(&mut self,size:usize) -> Result<String,Error>;

    #[cfg(feature="codec")]
    fn read_local_string(&mut self,size:usize,code: CodeType) -> Result<String,Error>;

    /// skip size byte
    fn skip_ptr(&mut self,size:usize) -> Result<usize,Error>; 
}

pub struct BytesReader {
    buffer: Vec<u8>,
    ptr: usize,
    endian: Endian,
}

#[cfg(feature="stream")]
pub struct StreamReader<R> {
    reader: R,
    endian: Endian,
}

impl BytesReader {
    fn system_endian() -> Endian {
        if cfg!(tarread_endian = "big") {
            Endian::BigEndian
        } else {
            Endian::LittleEndtian
        }
    }

    pub fn new(buffer:&[u8]) -> Self {
        Self{
            buffer:buffer.to_vec(),
            ptr: 0,
            endian: Self::system_endian(),
        } 
    }

    pub fn from_vec(buffer:Vec<u8>) -> Self { 
        Self{
            buffer: buffer,
            ptr: 0,
            endian: Self::system_endian(),
        } 

    }

    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    pub fn endian(&self) -> Endian {
        self.endian
    }

    fn check_bound(&mut self,size:usize) -> Result<(),Error> {
        if self.ptr + size > self.buffer.len() {
            let s = format!("ountbound call ptr {} + {} but buffer length {}",self.ptr,size,&self.buffer.len());
            Err( Error::new(ErrorKind::Other, s))
        } else {
            Ok(())
        }
    }
}

impl BinaryReader for BytesReader {
    fn read_byte(&mut self) -> Result<u8,Error>{
        self.check_bound(1)?;
        let b = &self.buffer[self.ptr];
        self.ptr += 1;
        Ok(*b)
    }
    fn read_u8(&mut self) -> Result<u8,Error>{
        self.read_byte()
    }

    fn read_i8(&mut self) -> Result<i8,Error>{
        Ok(self.read_byte()? as i8)
    }

    fn read_bytes_as_vec(&mut self,len: usize) -> Result<Vec<u8>,Error>{
        self.check_bound(len)?;
        let mut c:Vec<u8> = Vec::new();
        for i in 0..len {
            c.push(self.buffer[self.ptr + i]);
        }
        self.ptr += len;
        Ok(c)
    }

    fn read_bytes_no_move(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let len = if self.buffer.len() <= self.ptr + len 
                { self.buffer.len() - self.ptr } else {len};
        let mut c:Vec<u8> = Vec::new();
        for i in 0..len {
            c.push(self.buffer[self.ptr + i]);
        }
        Ok(c)
    }


    fn read_u16(&mut self) -> Result<u16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u16_be()
            },
            Endian::LittleEndtian => {
                self.read_u16_le()
            }
        }
    }

    fn read_u32(&mut self) ->  Result<u32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u32_be()
            },
            Endian::LittleEndtian => {
                self.read_u32_le()
            }
        }
    }

    fn read_u64(&mut self) -> Result<u64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u64_be()
            },
            Endian::LittleEndtian => {
                self.read_u64_le()
            }
        }
    }

    fn read_u128(&mut self) -> Result<u128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u128_be()
            },
            Endian::LittleEndtian => {
                self.read_u128_le()
            }
        }
    }

    fn read_i16(&mut self) -> Result<i16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i16_be()
            },
            Endian::LittleEndtian => {
                self.read_i16_le()
            }
        }
    }

    fn read_i32(&mut self) -> Result<i32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i32_be()
            },
            Endian::LittleEndtian => {
                self.read_i32_le()
            }
        }
    }

    fn read_i64(&mut self) -> Result<i64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i64_be()
            },
            Endian::LittleEndtian => {
                self.read_i64_le()
            }
        }
    }

    fn read_i128(&mut self) -> Result<i128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i128_be()
            },
            Endian::LittleEndtian => {
                self.read_i128_le()
            }
        }
    }

    fn read_f32(&mut self) -> Result<f32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f32_be()
            },
            Endian::LittleEndtian => {
                self.read_f32_le()
            }
        }
    }

    fn read_f64(&mut self) -> Result<f64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f64_be()
            },
            Endian::LittleEndtian => {
                self.read_f64_le()
            }
        }
    }

    
    fn read_u16_be(&mut self) -> Result<u16,Error>{
        let len = 2;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &&self.buffer;
        let array = [buf[ptr] ,buf[ptr+1]];
        Ok(u16::from_be_bytes(array))
    }

    fn read_u32_be(&mut self) -> Result<u32,Error>{
        let len = 4;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3]];
        Ok(u32::from_be_bytes(array))
    }

    
    fn read_u64_be(&mut self) -> Result<u64,Error>{
        let len = 8;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7]];
        Ok(u64::from_be_bytes(array))
    }

    fn read_u128_be(&mut self) -> Result<u128,Error>{
        let len = 16;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7],
                     buf[ptr+8] ,buf[ptr+9],buf[ptr+10] ,buf[ptr+11],
                     buf[ptr+12] ,buf[ptr+13],buf[ptr+14] ,buf[ptr+15]];
        Ok(u128::from_be_bytes(array))
    }

    fn read_i16_be(&mut self) -> Result<i16,Error>{
        let len = 2;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1]];
        Ok(i16::from_be_bytes(array))
    }

    fn read_i32_be(&mut self) -> Result<i32,Error>{
        let len = 4;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3]];
        Ok(i32::from_be_bytes(array))
    }

    fn read_i64_be(&mut self) -> Result<i64,Error>{
        let len = 8;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7]];
        Ok(i64::from_be_bytes(array))
    }

    fn read_i128_be(&mut self) -> Result<i128,Error>{
        let len = 16;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7],
                     buf[ptr+8] ,buf[ptr+9],buf[ptr+10] ,buf[ptr+11],
                     buf[ptr+12] ,buf[ptr+13],buf[ptr+14] ,buf[ptr+15]];
        Ok(i128::from_be_bytes(array))
    }

    fn read_f32_be(&mut self) -> Result<f32,Error>{
        let len = 4;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;

        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3]];
        Ok(f32::from_be_bytes(array))
    }

    fn read_f64_be(&mut self) -> Result<f64,Error>{
        let len = 8;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7]];
        Ok(f64::from_be_bytes(array))
    }
    
    fn read_u16_le(&mut self) -> Result<u16,Error>{
        let len = 2;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1]];
        Ok(u16::from_le_bytes(array))
    }

    fn read_u32_le(&mut self) -> Result<u32,Error>{
        let len = 4;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3]];
        Ok(u32::from_le_bytes(array))
    }

    
    fn read_u64_le(&mut self) -> Result<u64,Error>{
        let len = 8;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7]];
        Ok(u64::from_le_bytes(array))
    }

    fn read_u128_le(&mut self) -> Result<u128,Error>{
        let len = 16;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7],
                     buf[ptr+8] ,buf[ptr+9],buf[ptr+10] ,buf[ptr+11],
                     buf[ptr+12] ,buf[ptr+13],buf[ptr+14] ,buf[ptr+15]];
        Ok(u128::from_le_bytes(array))
    }

    fn read_i16_le(&mut self) -> Result<i16,Error>{
        let len = 2;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1]];
        Ok(i16::from_le_bytes(array))
    }

    fn read_i32_le(&mut self) -> Result<i32,Error>{
        let len = 4;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3]];
        Ok(i32::from_le_bytes(array))
    }

    fn read_i64_le(&mut self) -> Result<i64,Error>{
        let len = 8;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7]];
        Ok(i64::from_le_bytes(array))
    }

    fn read_i128_le(&mut self) -> Result<i128,Error>{
        let len = 16;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;
        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7],
                     buf[ptr+8] ,buf[ptr+9],buf[ptr+10] ,buf[ptr+11],
                     buf[ptr+12] ,buf[ptr+13],buf[ptr+14] ,buf[ptr+15]];
        Ok(i128::from_le_bytes(array))
    }

    fn read_f32_le(&mut self) -> Result<f32,Error>{
        let len = 4;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;

        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3]];
        Ok(f32::from_le_bytes(array))
    }

    fn read_f64_le(&mut self) -> Result<f64,Error>{
        let len = 8;
        self.check_bound(len)?;
        let ptr = self.ptr;
        self.ptr += len;
        let buf = &self.buffer;

        let array = [buf[ptr] ,buf[ptr+1],buf[ptr+2] ,buf[ptr+3],
                     buf[ptr+4] ,buf[ptr+5],buf[ptr+6] ,buf[ptr+7]];
        Ok(f64::from_le_bytes(array))
    }

    /// read until \0, but skip size byte
    fn read_ascii_string(&mut self,size:usize) -> Result<String,Error>{
        self.check_bound(size)?;
        let ptr = self.ptr;
        self.ptr += size;
        let buf = &self.buffer;
        let mut s = Vec::new();
        for i in 0..size {
            if buf[ptr + i] == 0 {break;}
            s.push(buf[ptr + i]);
        }
        let res = String::from_utf8(s);
        match res {
            Ok(strings) => {
                return Ok(strings);
            },
            _ => {
                let err = "This string can not read";
                return Err(Error::new(ErrorKind::Other,err));
            }
        }
    }

    fn read_utf8_string(&mut self,size:usize) -> Result<String,Error>{
        self.check_bound(size)?;
        let ptr = self.ptr;
        self.ptr += size;
        let buf = &self.buffer;
        let mut s = Vec::new();
        for i in 0..size {
            s.push(buf[ptr + i]);
        }
        let res = String::from_utf8(s);
        match res {
            Ok(strings) => {
                return Ok(strings);
            },
            _ => {
                let err = "This string can not read";
                return Err(Error::new(ErrorKind::Other,err));
            }
        }
    }

    #[cfg(feature="codec")]
    fn read_local_string(&mut self,size:usize,code: CodeType) -> Result<String,Error>{
        self.check_bound(size)?;
        self.ptr += size;
        Err(Error::new(ErrorKind::Other,"No impl"));
    }

    /// skip size byte
    fn skip_ptr(&mut self,size:usize) -> Result<usize,Error>{
        self.check_bound(size)?;
        self.ptr += size;
        Ok(size)
    }
}


#[cfg(feature="stream")]
impl<R:BufRead> StreamReader<R> {
    fn system_endian() -> Endian {
        if cfg!(tarread_endian = "big") {
            Endian::BigEndian
        } else {
            Endian::LittleEndtian
        }
    }

    pub fn new(reader: R) -> StreamReader<R> {
        StreamReader {
            reader: reader,
            endian: Self::system_endian(),
        }
    }

    
    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    pub fn endian(&self) -> Endian {
        self.endian
    }
    
}

#[cfg(feature="stream")]
impl<R:BufRead> BinaryReader for StreamReader<R> {
    fn read_byte(&mut self) -> Result<u8,Error>{
        let mut buffer = [0; 1];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }
    fn read_u8(&mut self) -> Result<u8,Error>{
        self.read_byte()
    }

    fn read_i8(&mut self) -> Result<i8,Error>{
        Ok(self.read_byte()? as i8)
    }

    fn read_bytes_as_vec(&mut self,len: usize) -> Result<Vec<u8>,Error>{
        let mut array: Vec<u8> = (0..len).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;
        Ok(array)
    }

    fn read_bytes_no_move(&mut self,len: usize) -> Result<Vec<u8>,Error> {
        let buffer = self.reader.fill_buf()?;
        if buffer.len() <= len {
            let err = "Data shotage";
            return Err(Error::new(ErrorKind::Other,err));
        }
        let array: Vec<u8> = (0..len).map(|i| buffer[i]).collect();
        Ok(array)
    }

    fn read_u16(&mut self) -> Result<u16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u16_be()
            },
            Endian::LittleEndtian => {
                self.read_u16_le()
            }
        }
    }

    fn read_u32(&mut self) ->  Result<u32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u32_be()
            },
            Endian::LittleEndtian => {
                self.read_u32_le()
            }
        }
    }

    fn read_u64(&mut self) -> Result<u64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u64_be()
            },
            Endian::LittleEndtian => {
                self.read_u64_le()
            }
        }
    }

    fn read_u128(&mut self) -> Result<u128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u128_be()
            },
            Endian::LittleEndtian => {
                self.read_u128_le()
            }
        }
    }

    fn read_i16(&mut self) -> Result<i16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i16_be()
            },
            Endian::LittleEndtian => {
                self.read_i16_le()
            }
        }
    }

    fn read_i32(&mut self) -> Result<i32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i32_be()
            },
            Endian::LittleEndtian => {
                self.read_i32_le()
            }
        }
    }

    fn read_i64(&mut self) -> Result<i64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i64_be()
            },
            Endian::LittleEndtian => {
                self.read_i64_le()
            }
        }
    }

    fn read_i128(&mut self) -> Result<i128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i128_be()
            },
            Endian::LittleEndtian => {
                self.read_i128_le()
            }
        }
    }

    fn read_f32(&mut self) -> Result<f32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f32_be()
            },
            Endian::LittleEndtian => {
                self.read_f32_le()
            }
        }
    }

    fn read_f64(&mut self) -> Result<f64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f64_be()
            },
            Endian::LittleEndtian => {
                self.read_f64_le()
            }
        }
    }

    
    fn read_u16_be(&mut self) -> Result<u16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(u16::from_be_bytes(array))
    }

    fn read_u32_be(&mut self) -> Result<u32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(u32::from_be_bytes(array))
    }

    
    fn read_u64_be(&mut self) -> Result<u64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(u64::from_be_bytes(array))
    }

    fn read_u128_be(&mut self) -> Result<u128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(u128::from_be_bytes(array))
    }

    fn read_i16_be(&mut self) -> Result<i16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(i16::from_be_bytes(array))
    }

    fn read_i32_be(&mut self) -> Result<i32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(i32::from_be_bytes(array))
    }

    fn read_i64_be(&mut self) -> Result<i64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(i64::from_be_bytes(array))
    }

    fn read_i128_be(&mut self) -> Result<i128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(i128::from_be_bytes(array))
    }

    fn read_f32_be(&mut self) -> Result<f32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(f32::from_be_bytes(array))
    }

    fn read_f64_be(&mut self) -> Result<f64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(f64::from_be_bytes(array))
    }
    
    fn read_u16_le(&mut self) -> Result<u16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(u16::from_le_bytes(array))
    }

    fn read_u32_le(&mut self) -> Result<u32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(u32::from_le_bytes(array))
    }

    
    fn read_u64_le(&mut self) -> Result<u64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(u64::from_le_bytes(array))
    }

    fn read_u128_le(&mut self) -> Result<u128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(u128::from_le_bytes(array))
    }

    fn read_i16_le(&mut self) -> Result<i16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(i16::from_le_bytes(array))
    }

    fn read_i32_le(&mut self) -> Result<i32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(i32::from_le_bytes(array))
    }

    fn read_i64_le(&mut self) -> Result<i64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(i64::from_le_bytes(array))
    }

    fn read_i128_le(&mut self) -> Result<i128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(i128::from_le_bytes(array))
    }

    fn read_f32_le(&mut self) -> Result<f32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(f32::from_le_bytes(array))
    }

    fn read_f64_le(&mut self) -> Result<f64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(f64::from_le_bytes(array))
    }


    fn read_ascii_string(&mut self,size:usize) -> Result<String,Error>{
        let mut array :Vec<u8> = (0..size).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;

        let buf = &array;
        let mut s = Vec::new();
        for i in 0..size {
            if buf[i] == 0 {break;}
            s.push(buf[i]);
        }
        let res = String::from_utf8(s);
        match res {
            Ok(strings) => {
                return Ok(strings);
            },
            _ => {
                let err = "This string can not read";
                return Err(Error::new(ErrorKind::Other,err));
            }
        }
    }


    fn read_utf8_string(&mut self,size:usize) -> Result<String,Error>{
        let mut array :Vec<u8> = (0..size).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;

        let buf = &array;
        let mut s = Vec::new();
        for i in 0..size {
            s.push(buf[i]);
        }
        let res = String::from_utf8(s);
        match res {
            Ok(strings) => {
                return Ok(strings);
            },
            _ => {
                let err = "This string can not read";
                return Err(Error::new(ErrorKind::Other,err));
            }
        }
    }

    #[cfg(feature="codec")]
    fn read_local_string(&mut self,size:usize,code: CodeType) -> Result<String,Error>{
        let mut array :Vec<u8> = (0..size).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;

        let buf = &array;
        let mut s = Vec::new();
        for i in 0..size {
            if buf[i] == 0 {break;}
            s.push(buf[i]);
        }
        let res = String::from_utf8(s);
        match res {
            Ok(strings) => {
                return Ok(strings);
            },
            _ => {
                let err = "This string can not read";
                return Err(Error::new(ErrorKind::Other,err));
            }
        }
    }

    /// skip size byte
    fn skip_ptr(&mut self,size:usize) -> Result<usize,Error>{
        let mut array :Vec<u8> = (0..size).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;
        Ok(size)
    }
}
