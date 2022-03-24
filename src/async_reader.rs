use async_trait::async_trait;
use std::io::prelude::BufRead;
use tokio::io::AsyncRead;
use tokio::io::Error;
use tokio::io::ErrorKind;
use crate::Endian;

#[async_trait]
pub trait AsyncBinaryReader {
    async fn read_byte(&mut self) -> Result<u8,Error>;
    async fn read_u8(&mut self) -> Result<u8,Error>;
//    async fn read_bytes(&mut self,len: usize) -> Result<&[u8],Error>;
    async fn read_bytes_as_vec(&mut self,len: usize) -> Result<Vec<u8>,Error>;
    async fn read_bytes_no_move(&mut self,len: usize) -> Result<Vec<u8>,Error>;

    async fn read_u16(&mut self) -> Result<u16,Error>;
    async fn read_u32(&mut self) -> Result<u32,Error>;
    async fn read_u64(&mut self) -> Result<u64,Error>;
    async fn read_u128(&mut self) -> Result<u128,Error>;
    async fn read_i8(&mut self) -> Result<i8,Error>;
    async fn read_i16(&mut self) -> Result<i16,Error>;
    async fn read_i32(&mut self) -> Result<i32,Error>;
    async fn read_i64(&mut self) -> Result<i64,Error>;
    async fn read_i128(&mut self) -> Result<i128,Error>;

    async fn read_f32(&mut self) -> Result<f32,Error>;
    async fn read_f64(&mut self) -> Result<f64,Error>;

    async fn read_u16_be(&mut self) -> Result<u16,Error>;
    async fn read_u32_be(&mut self) -> Result<u32,Error>;
    async fn read_u64_be(&mut self) -> Result<u64,Error>;
    async fn read_u128_be(&mut self) -> Result<u128,Error>;
    async fn read_i16_be(&mut self) -> Result<i16,Error>;
    async fn read_i32_be(&mut self) -> Result<i32,Error>;
    async fn read_i64_be(&mut self) -> Result<i64,Error>;
    async fn read_i128_be(&mut self) -> Result<i128,Error>;

    async fn read_f32_be(&mut self) -> Result<f32,Error>;
    async fn read_f64_be(&mut self) -> Result<f64,Error>;

    async fn read_u16_le(&mut self) -> Result<u16,Error>;
    async fn read_u32_le(&mut self) -> Result<u32,Error>;
    async fn read_u64_le(&mut self) -> Result<u64,Error>;
    async fn read_u128_le(&mut self) -> Result<u128,Error>;
    async fn read_i16_le(&mut self) -> Result<i16,Error>;
    async fn read_i32_le(&mut self) -> Result<i32,Error>;
    async fn read_i64_le(&mut self) -> Result<i64,Error>;
    async fn read_i128_le(&mut self) -> Result<i128,Error>;

    async fn read_f32_le(&mut self) -> Result<f32,Error>;
    async fn read_f64_le(&mut self) -> Result<f64,Error>;

    /// read until \0, but skip size byte
    async fn read_ascii_string(&mut self,size:usize) -> Result<String,Error>;

    async fn read_utf8_string(&mut self,size:usize) -> Result<String,Error>;

    #[cfg(feature="codec")]
    async fn read_local_string(&mut self,size:usize,code: CodeType) -> Result<String,Error>;

    /// skip size byte
    async fn skip_ptr(&mut self,size:usize) -> Result<usize,Error>; 
}

pub struct AsyncByteReader<R> {
    reader: R,
    endian: Endian,
}

impl<R:AsyncRead>  AsyncByteReader<R> {
   fn system_endian() -> Endian {
        if cfg!(tarread_endian = "big") {
            Endian::BigEndian
        } else {
            Endian::LittleEndtian
        }
    }

    pub fn new(reader: R) -> AsyncByteReader<R> {
        AsyncByteReader {
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

#[async_trait]
impl<R: BufRead + AsyncRead + Send> AsyncBinaryReader for AsyncByteReader<R> {
    
    async fn read_byte(&mut self) -> Result<u8,Error>{
        let mut buffer = [0; 1];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }
    
    async fn read_u8(&mut self) -> Result<u8,Error>{
        self.read_byte().await
    }

    async fn read_i8(&mut self) -> Result<i8,Error>{
        Ok(self.read_byte().await? as i8)
    }

    async fn read_bytes_as_vec(&mut self,len: usize) -> Result<Vec<u8>,Error>{
        let mut array: Vec<u8> = (0..len).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;
        Ok(array)
    }

    async fn read_bytes_no_move(&mut self,len: usize) -> Result<Vec<u8>,Error>{
        let buffer = self.reader.fill_buf()?;
        if buffer.len() <= len {
            let err = "Data shotage";
            return Err(Error::new(ErrorKind::Other,err));
        }
        let array: Vec<u8> = (0..len).map(|i| buffer[i]).collect();
        Ok(array)
    }

    async fn read_u16(&mut self) -> Result<u16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u16_be().await
            },
            Endian::LittleEndtian => {
                self.read_u16_le().await
            }
        }
    }

    async fn read_u32(&mut self) ->  Result<u32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u32_be().await
            },
            Endian::LittleEndtian => {
                self.read_u32_le().await
            }
        }
    }

    async fn read_u64(&mut self) -> Result<u64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u64_be().await
            },
            Endian::LittleEndtian => {
                self.read_u64_le().await
            }
        }
    }

    async fn read_u128(&mut self) -> Result<u128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_u128_be().await
            },
            Endian::LittleEndtian => {
                self.read_u128_le().await
            }
        }
    }

    async fn read_i16(&mut self) -> Result<i16,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i16_be().await
            },
            Endian::LittleEndtian => {
                self.read_i16_le().await
            }
        }
    }

    async fn read_i32(&mut self) -> Result<i32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i32_be().await
            },
            Endian::LittleEndtian => {
                self.read_i32_le().await
            }
        }
    }

    async fn read_i64(&mut self) -> Result<i64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i64_be().await
            },
            Endian::LittleEndtian => {
                self.read_i64_le().await
            }
        }
    }

    async fn read_i128(&mut self) -> Result<i128,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_i128_be().await
            },
            Endian::LittleEndtian => {
                self.read_i128_le().await
            }
        }
    }

    async fn read_f32(&mut self) -> Result<f32,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f32_be().await
            },
            Endian::LittleEndtian => {
                self.read_f32_le().await
            }
        }
    }

    async fn read_f64(&mut self) -> Result<f64,Error>{
        match self.endian {
            Endian::BigEndian => {
                self.read_f64_be().await
            },
            Endian::LittleEndtian => {
                self.read_f64_le().await
            }
        }
    }

    
    async fn read_u16_be(&mut self) -> Result<u16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(u16::from_be_bytes(array))
    }

    async fn read_u32_be(&mut self) -> Result<u32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(u32::from_be_bytes(array))
    }

    
    async fn read_u64_be(&mut self) -> Result<u64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(u64::from_be_bytes(array))
    }

    async fn read_u128_be(&mut self) -> Result<u128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(u128::from_be_bytes(array))
    }

    async fn read_i16_be(&mut self) -> Result<i16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(i16::from_be_bytes(array))
    }

    async fn read_i32_be(&mut self) -> Result<i32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(i32::from_be_bytes(array))
    }

    async fn read_i64_be(&mut self) -> Result<i64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(i64::from_be_bytes(array))
    }

    async fn read_i128_be(&mut self) -> Result<i128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(i128::from_be_bytes(array))
    }

    async fn read_f32_be(&mut self) -> Result<f32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(f32::from_be_bytes(array))
    }

    async fn read_f64_be(&mut self) -> Result<f64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(f64::from_be_bytes(array))
    }
    
    async fn read_u16_le(&mut self) -> Result<u16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(u16::from_le_bytes(array))
    }

    async fn read_u32_le(&mut self) -> Result<u32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(u32::from_le_bytes(array))
    }

    
    async fn read_u64_le(&mut self) -> Result<u64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(u64::from_le_bytes(array))
    }

    async fn read_u128_le(&mut self) -> Result<u128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(u128::from_le_bytes(array))
    }

    async fn read_i16_le(&mut self) -> Result<i16,Error>{
        let mut array = [0;2];
        self.reader.read_exact(&mut array)?;
        Ok(i16::from_le_bytes(array))
    }

    async fn read_i32_le(&mut self) -> Result<i32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(i32::from_le_bytes(array))
    }

    async fn read_i64_le(&mut self) -> Result<i64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(i64::from_le_bytes(array))
    }

    async fn read_i128_le(&mut self) -> Result<i128,Error>{
        let mut array = [0;16];
        self.reader.read_exact(&mut array)?;
        Ok(i128::from_le_bytes(array))
    }

    async fn read_f32_le(&mut self) -> Result<f32,Error>{
        let mut array = [0;4];
        self.reader.read_exact(&mut array)?;
        Ok(f32::from_le_bytes(array))
    }

    async fn read_f64_le(&mut self) -> Result<f64,Error>{
        let mut array = [0;8];
        self.reader.read_exact(&mut array)?;
        Ok(f64::from_le_bytes(array))
    }

    /// read until \0, but skip size byte
    async fn read_ascii_string(&mut self,size:usize) -> Result<String,Error>{
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

    async fn read_utf8_string(&mut self,size:usize) -> Result<String,Error>{
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
    async fn read_local_string(&mut self,size:usize,code: CodeType) -> Result<String,Error>{
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
    async fn skip_ptr(&mut self,size:usize) -> Result<usize,Error>{
        let mut array :Vec<u8> = (0..size).map(|_| 0).collect();
        self.reader.read_exact(&mut array)?;
        Ok(size)
    }
}

