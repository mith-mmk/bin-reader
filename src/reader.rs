//! A reader default uses system endianness
//! If you will use another endianness,use set_endian.
//! 0.0.9 enable Stream Reader is default but not enable wasm

use std::io::Read;
use std::io::Cursor;
use crate::Endian;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Error;
use std::io::ErrorKind;

#[cfg(feature="codec")]
use encoding_rs::*;

#[cfg(not(target_family = "wasm"))]
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
    fn set_endian(&mut self, endian: Endian);
    fn endian(&self) -> Endian;
    
    fn read_byte(&mut self) -> Result<u8,Error>;
    fn read_u8(&mut self) -> Result<u8,Error>;
    fn read_bytes(&mut self,array: &mut [u8]) -> Result<(),Error>;
//    fn read_bytes(&mut self,len: usize) -> Result<&[u8],Error>;
    fn read_bytes_as_vec(&mut self,len: usize) -> Result<Vec<u8>,Error>;

    /// read_bytes_no_move does not move offset after read_bytes.
    /// 
    /// Assumed to be used for header checks.
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

    fn read_ascii_string(&mut self,size:usize) -> Result<String,Error>;

    /// read_utf16_string for utf16 string. use endien

    fn read_utf16_string(&mut self,size:usize) -> Result<String,Error>;

    fn read_utf16be_string(&mut self,size:usize) -> Result<String,Error> {
        self.set_endian(Endian::BigEndian);
        self.read_utf16_string(size)
    }

    fn read_utf16le_string(&mut self,size:usize) -> Result<String,Error> {
        self.set_endian(Endian::LittleEndian);
        self.read_utf16_string(size)
    }

    fn read_utf8_string(&mut self,size:usize) -> Result<String,Error>;

    #[cfg(feature="codec")]
    fn read_local_string(&mut self,size:usize,code: CodeType) -> Result<String,Error>;

    /// skip size byte
    fn skip_ptr(&mut self,size:usize) -> Result<usize,Error>;

    fn offset(&mut self) -> Result<u64,Error>;
    fn seek(&mut self,seek: SeekFrom) -> Result<u64,Error>;
}

/// BytesReader from creating Slice `&[u8]` or `Vec<u8>`,
/// no use Read trait
#[derive(Debug,Clone)]
pub struct BytesReader {
    buffer: Vec<u8>,
    ptr: usize,
    endian: Endian,
}

#[cfg(not(target_family = "wasm"))]
/// 0.0.8 Enable support for target_family other than "wasm", feature "stream" is disabled.
///
/// StreamReader from creating BufRead
/// use BufRead trait
#[derive(Copy,Debug,Clone)]
pub struct StreamReader<R> {
    reader: R,
    endian: Endian,
}

impl BytesReader {
    pub fn new(buffer:&[u8]) -> Self {
        Self{
            buffer:buffer.to_vec(),
            ptr: 0,
            endian: crate::system_endian(),
        } 
    }

    pub fn from_vec(buffer:Vec<u8>) -> Self { 
        Self{
            buffer: buffer,
            ptr: 0,
            endian: crate::system_endian(),
        } 
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


#[cfg(not(target_family = "wasm"))]
impl<R:BufRead + Seek> StreamReader<R> {
    pub fn new(reader: R) -> StreamReader<R> {
        StreamReader {
            reader: reader,
            endian: crate::system_endian(),
        }
    }
}




#[cfg(not(target_family = "wasm"))]
impl<R> From<R> for StreamReader<Cursor<R>> 
    where R: Read {

    fn from(reader: R) -> Self {
        let reader = Cursor::new(reader);
        Self {
            reader: reader,
            endian: crate::system_endian(),
        }

    }
}


impl BinaryReader for BytesReader {
    fn offset(&mut self) -> Result<u64,Error> {
        return Ok(self.ptr as u64)
    }

    fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    fn endian(&self) -> Endian {
        self.endian
    }

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

    fn read_bytes(&mut self,array: &mut [u8]) -> Result<(),Error> {
        let len = array.len();
        self.check_bound(len)?;
        for i in 0..len {
            array[i] = self.buffer[self.ptr + i];
        }
        self.ptr += len;
        Ok(())
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

    // This function read bytes, but it does not move pointer.
    /// ```
    /// use bin_rs::reader::*;
    /// use std::io::Error;
    /// fn test() ->  Result<(),Error> {
    ///    let buffer = b"Hello World!";
    ///    let mut reader = BytesReader::new(buffer);
    ///    let buffer1 = reader.read_bytes_no_move(4)?;
    /// // assert_eq!(buffer1,b"Hell");
    ///    let buffer1 = reader.read_bytes_as_vec(4)?;
    /// // assert_eq!(buffer1,b"Hell");
    ///    let buffer1 = reader.read_bytes_as_vec(4)?;
    /// // assert_eq!(buffer1,b"o Wo");
    ///    return Ok(())
    /// }
    /// ```
    /// 
    fn read_bytes_no_move(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        self.check_bound(len)?;
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
            Endian::LittleEndian => {
                self.read_u16_le()
            }
        }
    }

    fn read_u32(&mut self) ->  Result<u32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u32_be()
            },
            Endian::LittleEndian => {
                self.read_u32_le()
            }
        }
    }

    fn read_u64(&mut self) -> Result<u64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u64_be()
            },
            Endian::LittleEndian => {
                self.read_u64_le()
            }
        }
    }

    fn read_u128(&mut self) -> Result<u128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u128_be()
            },
            Endian::LittleEndian => {
                self.read_u128_le()
            }
        }
    }

    fn read_i16(&mut self) -> Result<i16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i16_be()
            },
            Endian::LittleEndian => {
                self.read_i16_le()
            }
        }
    }

    fn read_i32(&mut self) -> Result<i32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i32_be()
            },
            Endian::LittleEndian => {
                self.read_i32_le()
            }
        }
    }

    fn read_i64(&mut self) -> Result<i64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i64_be()
            },
            Endian::LittleEndian => {
                self.read_i64_le()
            }
        }
    }

    fn read_i128(&mut self) -> Result<i128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i128_be()
            },
            Endian::LittleEndian => {
                self.read_i128_le()
            }
        }
    }

    fn read_f32(&mut self) -> Result<f32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f32_be()
            },
            Endian::LittleEndian => {
                self.read_f32_le()
            }
        }
    }

    fn read_f64(&mut self) -> Result<f64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f64_be()
            },
            Endian::LittleEndian => {
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
    /// ```
    /// use bin_rs::reader::*;
    /// use std::io::Error;
    /// 
    /// fn test() -> Result<String,Error> {
    ///   let buffer = b"Hello World!\01234";
    ///   let mut reader = BytesReader::new(buffer);
    ///   let r = reader.read_ascii_string("Hello World!\01234".len())?;  // after \0 is trim
    ///   //assert_eq!(r ,"Hello World!");
    ///   return Ok(r)
    /// }
    /// ```
    fn read_utf16_string(&mut self,size:usize) -> Result<String,Error> {
        let endian = self.endian;
        self.check_bound(size * 2)?;
        let ptr = self.ptr;
        self.ptr += size * 2;
        let buf = &self.buffer;
        let mut s = Vec::new();
        for i in 0..size {
            let array = [buf[ptr + i * 2] ,buf[ptr + i * 2 + 1]];
            let c = match endian {
                Endian::BigEndian => {
                    u16::from_be_bytes(array)
                },
                Endian::LittleEndian => {
                    u16::from_le_bytes(array)
                }
                
            };
            if c == 0 {break;}
            s.push(c);
        }
        let res = String::from_utf16(&s);
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

    /// read end of size,not skip 0x00 marker
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

    /// skip_ptr skips offset size bytes
    fn skip_ptr(&mut self,size:usize) -> Result<usize,Error>{
        self.check_bound(size)?;
        self.ptr += size;
        Ok(size)
    }
    
    fn seek(&mut self, seek: SeekFrom) -> std::result::Result<u64, Error> {
        match seek {
            SeekFrom::Start(pos)=>{
                if pos >= usize::MAX as u64 {
                    let s = format!("BytesReader max offset is usize length but set {}",pos);
                    return Err( Error::new(ErrorKind::Other, s))
                } else if pos >= self.buffer.len() as u64 {
                    let s = format!("set offset {},but buffer length is{}",pos,self.buffer.len());
                    return Err( Error::new(ErrorKind::Other, s))
                }
                self.ptr = pos as usize;
                Ok(self.ptr as u64)
            },
            SeekFrom::End(pos_)=>{
                let pos = self.buffer.len() as i64 + pos_;
                if pos < 0 || pos >= (self.buffer.len() as i64) {
                    let s = format!("set offset {},but buffer length is {}",pos,self.buffer.len());
                    return Err( Error::new(ErrorKind::Other, s))
                }
                self.ptr = pos as usize;
                Ok(self.ptr as u64)
            },
            SeekFrom::Current(pos)=>{
                let ptr = (self.ptr as i64) + pos;
                if ptr >= usize::MAX as i64 {
                    let s = format!("BytesReader max offset is usize length but set {}",ptr);
                    return Err( Error::new(ErrorKind::Other, s))
                } else if self.buffer.len() as i64 <= ptr || ptr < 0 {
                    let s = format!("set offset {},but buffer length is{}",ptr,self.buffer.len());
                    return Err( Error::new(ErrorKind::Other, s))
                }
                self.ptr = ptr as usize;
                Ok(self.ptr as u64)
            },
        }
    }
}


#[cfg(not(target_family = "wasm"))]
impl<R:BufRead+Seek> BinaryReader for StreamReader<R> {

    fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    fn endian(&self) -> Endian {
        self.endian
    }

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

    fn read_bytes(&mut self, array: &mut [u8]) -> std::result::Result<(), Error> {
        self.reader.read_exact(array)?;
        Ok(())
    }


    fn read_bytes_as_vec(&mut self,len: usize) -> Result<Vec<u8>,Error>{
        let mut array: Vec<u8> = (0..len).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;
        Ok(array)
    }

    // This function read bytes and does not move pointer.
    // However it's behavior dependences read buffer size.
    fn read_bytes_no_move(&mut self,len: usize) -> Result<Vec<u8>,Error> {
        let buffer = self.reader.fill_buf()?;
        if buffer.len() < len {
            let err = format!("Data shotage,request {} but read {} bytes",len,buffer.len());
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
            Endian::LittleEndian => {
                self.read_u16_le()
            }
        }
    }

    fn read_u32(&mut self) ->  Result<u32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u32_be()
            },
            Endian::LittleEndian => {
                self.read_u32_le()
            }
        }
    }

    fn read_u64(&mut self) -> Result<u64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u64_be()
            },
            Endian::LittleEndian => {
                self.read_u64_le()
            }
        }
    }

    fn read_u128(&mut self) -> Result<u128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u128_be()
            },
            Endian::LittleEndian => {
                self.read_u128_le()
            }
        }
    }

    fn read_i16(&mut self) -> Result<i16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i16_be()
            },
            Endian::LittleEndian => {
                self.read_i16_le()
            }
        }
    }

    fn read_i32(&mut self) -> Result<i32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i32_be()
            },
            Endian::LittleEndian => {
                self.read_i32_le()
            }
        }
    }

    fn read_i64(&mut self) -> Result<i64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i64_be()
            },
            Endian::LittleEndian => {
                self.read_i64_le()
            }
        }
    }

    fn read_i128(&mut self) -> Result<i128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i128_be()
            },
            Endian::LittleEndian => {
                self.read_i128_le()
            }
        }
    }

    fn read_f32(&mut self) -> Result<f32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f32_be()
            },
            Endian::LittleEndian => {
                self.read_f32_le()
            }
        }
    }

    fn read_f64(&mut self) -> Result<f64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f64_be()
            },
            Endian::LittleEndian => {
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

    fn offset(&mut self) -> std::result::Result<u64,Error> {
        self.reader.seek(SeekFrom::Current(0))
    }

    fn seek(&mut self, seek: std::io::SeekFrom) -> std::result::Result<u64, Error> {
        self.reader.seek(seek)
    }

    fn read_utf16_string(&mut self,size:usize) -> Result<String,Error> {
        let endian = self.endian;
        let mut array :Vec<u8> = (0..size * 2).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;
        let buf = &array;
        let mut s = Vec::new();
        for i in 0..size {
            let array = [buf[i * 2] ,buf[i * 2 + 1]];
            let c = match endian {
                Endian::BigEndian => {
                    u16::from_be_bytes(array)
                },
                Endian::LittleEndian => {
                    u16::from_le_bytes(array)
                }
                
            };
            if c == 0 {break;}
            s.push(c);
        }
        let res = String::from_utf16(&s);
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
}