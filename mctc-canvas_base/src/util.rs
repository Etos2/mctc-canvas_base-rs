use std::io::Read;

use crate::error::PResult;

pub(crate) trait ReadExt {
    fn read_u8(&mut self) -> PResult<u8>;
    fn read_u16(&mut self) -> PResult<u16>;
    fn read_u32(&mut self) -> PResult<u32>;
    fn read_u64(&mut self) -> PResult<u64>;
    fn read_i64(&mut self) -> PResult<i64>;
    fn read_f32(&mut self) -> PResult<f32>;
    fn read_vec(&mut self, bytes: usize) -> PResult<Vec<u8>>;
}

#[inline]
fn read_array<const N: usize>(mut rdr: impl Read) -> PResult<[u8; N]> {
    let mut buf = [0u8; N];
    rdr.read_exact(&mut buf)?;
    Ok(buf)
}

impl<R: Read> ReadExt for R {
    #[inline]
    fn read_u8(&mut self) -> PResult<u8> {
        Ok(read_array::<1>(self)?[0])
    }

    #[inline]
    fn read_u16(&mut self) -> PResult<u16> {
        Ok(u16::from_le_bytes(read_array(self)?))
    }

    #[inline]
    fn read_u32(&mut self) -> PResult<u32> {
        Ok(u32::from_le_bytes(read_array(self)?))
    }

    #[inline]
    fn read_u64(&mut self) -> PResult<u64> {
        Ok(u64::from_le_bytes(read_array(self)?))
    }

    #[inline]
    fn read_i64(&mut self) -> PResult<i64> {
        Ok(i64::from_le_bytes(read_array(self)?))
    }

    #[inline]
    fn read_f32(&mut self) -> PResult<f32> {
        Ok(f32::from_le_bytes(read_array(self)?))
    }

    #[inline]
    fn read_vec(&mut self, bytes: usize) -> PResult<Vec<u8>> {
        let mut buf = vec![0; bytes];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}