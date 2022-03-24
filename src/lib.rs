//! Crate bin_rs is a Binary Read Crate.

pub mod reader;
#[cfg(feature="async")]
pub mod async_reader;

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

#[cfg(test)]
mod tests {
    use crate::reader::*;
    use crate::Endian;
    use std::error::Error;
    #[test]
    fn check_works() -> Result<(),std::io::Error> {
        let buffer : Vec<u8> = (0..255).map(|i| i).collect();
        let mut reader = BytesReader::from_vec(buffer);

        let r = reader.read_byte()?;
        assert_eq!(r , 0_u8);
        let r = reader.read_u8()?;
        assert_eq!(r , 1_u8);
        let r = reader.read_u16_be()?;
        assert_eq!(r , 0x0203);
        let r = reader.read_u16_le()?;
        assert_eq!(r , 0x0504);
        let r = reader.read_u32_be()?;
        assert_eq!(r , 0x06070809);
        let r = reader.read_u32_le()?;
        assert_eq!(r , 0x0d0c0b0a);
        reader.skip_ptr(2)?; // 0x0e 0x0f skip
        let r = reader.read_u64_be()?;
        assert_eq!(r , 0x1011121314151617);
        let r = reader.read_u64_le()?;
        assert_eq!(r , 0x1f1e1d1c1b1a1918);
        let r = reader.read_u128_be()?;
        assert_eq!(r , 0x202122232425262728292a2b2c2d2e2f);
        let r = reader.read_u128_le()?;
        assert_eq!(r , 0x3f3e3d3c3b3a39383736353433323130);

        let buffer : Vec<u8> = (0..32).map(|i| 255-i).collect();
        let mut reader = BytesReader::from_vec(buffer);
        let r = reader.read_i8()?;  // 0xff
        assert_eq!(r , -1);
        let r = reader.read_i16_be()?; // 0xfefd -> fefd
        assert_eq!(r , 	-259);
        let r = reader.read_i16_le()?; // 0xfcfb -> fbfc
        assert_eq!(r , -1028);
        let r = reader.read_i32_be()?; // 0xfaf9f8f7
        assert_eq!(r , -84281097);
        let r = reader.read_i32_le()?; // 0xf3f4f5f6
        assert_eq!(r , -202050058);

        let r = reader.read_bytes_as_vec(2)?;
        assert_eq!(r, [0xf2,0xf1]);

        let r = reader.read_i128_le();
        if r.is_err() {
            assert!(true)
        }

        let buffer : Vec<u8> = (0..16).map(|i| 255-i).collect();
        let mut reader = BytesReader::from_vec(buffer);
        let r = reader.read_i64_be()?;
        assert_eq!(r , -283686952306184);

        let r = reader.read_i64_le()?;
        assert_eq!(r , -1084818905618843913);

        let buffer = [0x41,0x89,0x85,0x1F];
        let mut reader = BytesReader::new(&buffer);
        let r = reader.read_f32_be()?;
        assert_eq!(r , 17.19);

        let buffer = [0x1F,0x85,0x89,0x41];
        let mut reader = BytesReader::new(&buffer);
        let r = reader.read_f32_le()?;
        assert_eq!(r , 17.19);

        let buffer = [0xC0,0x31,0x30,0xA3,0xD7,0x0A,0x3D,0x71];
        let mut reader = BytesReader::new(&buffer);
        let r = reader.read_f64_be()?;
        assert_eq!(r ,-17.19);
        let buffer = [0x71,0x3D,0x0A,0xD7,0xA3,0x30,0x31,0xC0];
        let mut reader = BytesReader::new(&buffer);
        let r = reader.read_f64_le()?;
        assert_eq!(r ,-17.19);

        let buffer = b"Hello World!";
        let mut reader = BytesReader::new(buffer);
        let buffer1 = reader.read_bytes_no_move(4)?;
        assert_eq!(buffer1,b"Hell");
        let buffer1 = reader.read_bytes_as_vec(4)?;
        assert_eq!(buffer1,b"Hell");
        let buffer1 = reader.read_bytes_as_vec(4)?;
        assert_eq!(buffer1,b"o Wo");

        let buffer = b"Hello World!\01234";
        let mut reader = BytesReader::new(buffer);
        let r = reader.read_ascii_string("Hello World!\01234".len())?;  // must after \0 is trim
        assert_eq!(r ,"Hello World!");
        let buffer = b"\xE3\x81\xB8\xE3\x82\x8D\xE3\x83\xBC\xE3\x82\x8F\xE3\x83\xBC\xE3\x82\x8B\xE3\x81\xA9\01234";
        let mut reader = BytesReader::new(buffer);
        let r = reader.read_utf8_string(23)?;
        assert_eq!(r ,"へろーわーるど\01");

        if cfg!(feature="codec") {
            // no impl reader.read_local_string
        }

        let buffer = [0x71,0x3D,0x0A,0xD7,0xA3,0x30,0x31,0xC0];
        let mut reader = BytesReader::new(&buffer);
        reader.set_endian(Endian::LittleEndtian);
        let r = reader.read_f64()?;
        assert_eq!(r ,-17.19);

        let buffer = [0xC0,0x31,0x30,0xA3,0xD7,0x0A,0x3D,0x71];
        let mut reader = BytesReader::new(&buffer);
        reader.set_endian(Endian::BigEndian);
        let r = reader.read_f64()?;
        assert_eq!(r ,-17.19);

        Ok(())
    }


    #[cfg(feature="stream")]
    #[test]
    fn check_stream () -> Result<(),std::io::Error> {
        use crate::reader::StreamReader;

        let buffer : Vec<u8> = (0..255).map(|i| i).collect();
        let mut reader = StreamReader::new(&*buffer);
    
        let r = reader.read_byte()?;
        assert_eq!(r , 0_u8);
        let r = reader.read_u8()?;
        assert_eq!(r , 1_u8);
        let r = reader.read_u16_be()?;
        assert_eq!(r , 0x0203);
        let r = reader.read_u16_le()?;
        assert_eq!(r , 0x0504);
        let r = reader.read_u32_be()?;
        assert_eq!(r , 0x06070809);
        let r = reader.read_u32_le()?;
        assert_eq!(r , 0x0d0c0b0a);
        reader.skip_ptr(2)?; // 0x0e 0x0f skip
        let r = reader.read_u64_be()?;
        assert_eq!(r , 0x1011121314151617);
        let r = reader.read_u64_le()?;
        assert_eq!(r , 0x1f1e1d1c1b1a1918);
        let r = reader.read_u128_be()?;
        assert_eq!(r , 0x202122232425262728292a2b2c2d2e2f);
        let r = reader.read_u128_le()?;
        assert_eq!(r , 0x3f3e3d3c3b3a39383736353433323130);
    
        let buffer : Vec<u8> = (0..32).map(|i| 255-i).collect();
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_i8()?;  // 0xff
        assert_eq!(r , -1);
        let r = reader.read_i16_be()?; // 0xfefd -> fefd
        assert_eq!(r , 	-259);
        let r = reader.read_i16_le()?; // 0xfcfb -> fbfc
        assert_eq!(r , -1028);
        let r = reader.read_i32_be()?; // 0xfaf9f8f7
        assert_eq!(r , -84281097);
        let r = reader.read_i32_le()?; // 0xf3f4f5f6
        assert_eq!(r , -202050058);
    
        let r = reader.read_bytes_as_vec(2)?;
        assert_eq!(r, [0xf2,0xf1]);
    
        let r = reader.read_i128_le();  // outbounds
        if r.is_err() {
            assert!(true)
        }
    
        let buffer : Vec<u8> = (0..16).map(|i| 255-i).collect();
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_i64_be()?;
        assert_eq!(r , -283686952306184);
    
        let r = reader.read_i64_le()?;
        assert_eq!(r , -1084818905618843913);
    
        let buffer = vec![0x41,0x89,0x85,0x1F];
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_f32_be()?;
        assert_eq!(r , 17.19);
    
        let buffer = vec![0x1F,0x85,0x89,0x41];
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_f32_le()?;
        assert_eq!(r , 17.19);
    
        let buffer = vec![0xC0,0x31,0x30,0xA3,0xD7,0x0A,0x3D,0x71];
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_f64_be()?;
        assert_eq!(r ,-17.19);
        let buffer = vec![0x71,0x3D,0x0A,0xD7,0xA3,0x30,0x31,0xC0];
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_f64_le()?;
        assert_eq!(r ,-17.19);
    
        let buffer = b"Hello World!".to_vec();
        let mut reader = StreamReader::new(&*buffer);
        let buffer1 = reader.read_bytes_no_move(4)?;
        assert_eq!(buffer1,b"Hell");
        let buffer1 = reader.read_bytes_as_vec(4)?;
        assert_eq!(buffer1,b"Hell");
        let buffer1 = reader.read_bytes_as_vec(4)?;
        assert_eq!(buffer1,b"o Wo");
    
        let buffer = b"Hello World!\01234".to_vec();
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_ascii_string("Hello World!\01234".len())?;  // must after \0 is trim
        assert_eq!(r ,"Hello World!");
        let buffer = b"\xE3\x81\xB8\xE3\x82\x8D\xE3\x83\xBC\xE3\x82\x8F\xE3\x83\xBC\xE3\x82\x8B\xE3\x81\xA9\01234".to_vec();
        let mut reader = StreamReader::new(&*buffer);
        let r = reader.read_utf8_string(23)?;
        assert_eq!(r ,"へろーわーるど\01");
    
        if cfg!(feature="codec") {
            // no impl reader.read_local_string
        }
    
        let buffer = [0x71,0x3D,0x0A,0xD7,0xA3,0x30,0x31,0xC0].to_vec();
        let mut reader = StreamReader::new(&*buffer);
        reader.set_endian(Endian::LittleEndtian);
        let r = reader.read_f64()?;
        assert_eq!(r ,-17.19);
    
        let buffer = [0xC0,0x31,0x30,0xA3,0xD7,0x0A,0x3D,0x71].to_vec();
        let mut reader = StreamReader::new(&*buffer);
        reader.set_endian(Endian::BigEndian);
        let r = reader.read_f64()?;
        assert_eq!(r ,-17.19);

    
        Ok(())
        
    }

    #[tokio::test]
    #[cfg(feature="async")]
    pub async fn check_async () -> Result<(),Box<dyn Error>> {
        use crate::async_reader::AsyncBytesReader;
        let buffer : Vec<u8> = (0..255).map(|i| i).collect();
        let mut reader = AsyncBytesReader::new(&*buffer);
    
        let r = reader.read_byte().await?;
        assert_eq!(r , 0_u8);
        let r = reader.read_u8().await?;
        assert_eq!(r , 1_u8);
        let r = reader.read_u16_be().await?;
        assert_eq!(r , 0x0203);
        let r = reader.read_u16_le().await?;
        assert_eq!(r , 0x0504);
        let r = reader.read_u32_be().await?;
        assert_eq!(r , 0x06070809);
        let r = reader.read_u32_le().await?;
        assert_eq!(r , 0x0d0c0b0a);
        reader.skip_ptr(2).await?; // 0x0e 0x0f skip
        let r = reader.read_u64_be().await?;
        assert_eq!(r , 0x1011121314151617);
        let r = reader.read_u64_le().await?;
        assert_eq!(r , 0x1f1e1d1c1b1a1918);
        let r = reader.read_u128_be().await?;
        assert_eq!(r , 0x202122232425262728292a2b2c2d2e2f);
        let r = reader.read_u128_le().await?;
        assert_eq!(r , 0x3f3e3d3c3b3a39383736353433323130);
    
        let buffer : Vec<u8> = (0..32).map(|i| 255-i).collect();
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_i8().await?;  // 0xff
        assert_eq!(r , -1);
        let r = reader.read_i16_be().await?; // 0xfefd -> fefd
        assert_eq!(r , 	-259);
        let r = reader.read_i16_le().await?; // 0xfcfb -> fbfc
        assert_eq!(r , -1028);
        let r = reader.read_i32_be().await?; // 0xfaf9f8f7
        assert_eq!(r , -84281097);
        let r = reader.read_i32_le().await?; // 0xf3f4f5f6
        assert_eq!(r , -202050058);
    
        let r = reader.read_bytes_as_vec(2).await?;
        assert_eq!(r, [0xf2,0xf1]);
    
        let r = reader.read_i128_le().await;  // outbounds
        if r.is_err() {
            assert!(true)
        }
    
        let buffer : Vec<u8> = (0..16).map(|i| 255-i).collect();
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_i64_be().await?;
        assert_eq!(r , -283686952306184);
    
        let r = reader.read_i64_le().await?;
        assert_eq!(r , -1084818905618843913);
    
        let buffer = vec![0x41,0x89,0x85,0x1F];
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_f32_be().await?;
        assert_eq!(r , 17.19);
    
        let buffer = vec![0x1F,0x85,0x89,0x41];
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_f32_le().await?;
        assert_eq!(r , 17.19);
    
        let buffer = vec![0xC0,0x31,0x30,0xA3,0xD7,0x0A,0x3D,0x71];
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_f64_be().await?;
        assert_eq!(r ,-17.19);
        let buffer = vec![0x71,0x3D,0x0A,0xD7,0xA3,0x30,0x31,0xC0];
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_f64_le().await?;
        assert_eq!(r ,-17.19);
    
        let buffer = b"Hello World!".to_vec();
        let mut reader = AsyncBytesReader::new(&*buffer);
        let buffer1 = reader.read_bytes_no_move(4).await?;
        assert_eq!(buffer1,b"Hell");
        let buffer1 = reader.read_bytes_as_vec(4).await?;
        assert_eq!(buffer1,b"Hell");
        let buffer1 = reader.read_bytes_as_vec(4).await?;
        assert_eq!(buffer1,b"o Wo");
    
        let buffer = b"Hello World!\01234".to_vec();
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_ascii_string("Hello World!\01234".len()).await?;  // must after \0 is trim
        assert_eq!(r ,"Hello World!");
        let buffer = b"\xE3\x81\xB8\xE3\x82\x8D\xE3\x83\xBC\xE3\x82\x8F\xE3\x83\xBC\xE3\x82\x8B\xE3\x81\xA9\01234".to_vec();
        let mut reader = AsyncBytesReader::new(&*buffer);
        let r = reader.read_utf8_string(23).await?;
        assert_eq!(r ,"へろーわーるど\01");
    
        if cfg!(feature="codec") {
            // no impl reader.read_local_string
        }
    
        let buffer = [0x71,0x3D,0x0A,0xD7,0xA3,0x30,0x31,0xC0].to_vec();
        let mut reader = AsyncBytesReader::new(&*buffer);
        reader.set_endian(Endian::LittleEndtian);
        let r = reader.read_f64().await?;
        assert_eq!(r ,-17.19);
    
        let buffer = [0xC0,0x31,0x30,0xA3,0xD7,0x0A,0x3D,0x71].to_vec();
        let mut reader = AsyncBytesReader::new(&*buffer);
        reader.set_endian(Endian::BigEndian);
        let r = reader.read_f64().await?;
        assert_eq!(r ,-17.19);
    
        Ok(())
        
    }
    
}
