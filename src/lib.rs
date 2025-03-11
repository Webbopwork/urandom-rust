use std::io;
use std::io::prelude::*;
use std::fs::File;


pub struct Urandom(File);

impl Urandom {
    pub fn new() -> io::Result<Self> {
        File::open("/dev/urandom").map(|f| Self(f))
    }
    pub fn read_buffer(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        self.0.read_exact(buffer)?;
        Ok(())
    }

    pub fn read_return(mut self, buffer: &mut [u8]) -> io::Result<&mut [u8]> {
        self.read_buffer(buffer)?;
        Ok(buffer)
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8; 1];
        self.read_buffer(&mut buffer)?;
        Ok(u8::from_le_bytes(buffer))
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0u8; 2];
        self.read_buffer(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0u8; 4];
        self.read_buffer(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    pub fn read_f32(&mut self) -> io::Result<f32> {
        let mut buffer = [0u8; 4];
        self.read_buffer(&mut buffer)?;
        Ok(f32::from_le_bytes(buffer))
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0u8; 8];
        self.read_buffer(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    pub fn read_usize(&mut self) -> io::Result<usize> {
        let mut buffer = [0u8; 8];
        self.read_buffer(&mut buffer)?;
        Ok(usize::from_le_bytes(buffer))
    }

    pub fn read_u128(&mut self) -> io::Result<u128> {
        let mut buffer = [0u8; 16];
        self.read_buffer(&mut buffer)?;
        Ok(u128::from_le_bytes(buffer))
    }
}

/*pub trait Urandom: Sized {
    /// Generates a random value.
    fn random(source: &mut (impl RandomSource + ?Sized)) -> Self;
}

macro_rules! impl_primitive {
    ($t:ty) => {
        impl Urandom for $t {
            //fn random(source: &mut (impl Urandom + ?Sized)) -> io::Result<Self> {
            fn random(source: mut self) -> io::Result<Self> {
                let mut buffer = (0 as Self).to_le_bytes();
                source.read_buffer(&mut buffer)?;
                Ok(Self::from_le_bytes(buffer))
            }
        }
    };
}

impl_primitive!(u16);*/


pub fn open_urandom() -> io::Result<File> {
    File::open("/dev/urandom")
}

pub fn read_urandom(urandom_file: &mut File, buffer: &mut [u8]) -> io::Result<()> {
    urandom_file.read_exact(buffer)?;
    Ok(())
}