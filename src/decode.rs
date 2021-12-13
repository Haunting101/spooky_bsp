use byteorder::{LittleEndian, ReadBytesExt};
use std::{io::Read, mem::MaybeUninit};

pub trait Decode
where
    Self: Sized,
{
    fn decode(reader: &mut impl Read) -> eyre::Result<Self>;
}

impl Decode for bool {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_i32::<LittleEndian>()? != 0)
    }
}

impl Decode for char {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(i32::decode(reader)? as u8 as char)
    }
}

impl Decode for i8 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_i8()?)
    }
}

impl Decode for u8 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_u8()?)
    }
}

impl Decode for i16 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_i16::<LittleEndian>()?)
    }
}

impl Decode for u16 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_u16::<LittleEndian>()?)
    }
}

impl Decode for i32 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_i32::<LittleEndian>()?)
    }
}

impl Decode for u32 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }
}

impl Decode for i64 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_i64::<LittleEndian>()?)
    }
}

impl Decode for u64 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }
}

impl Decode for f32 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_f32::<LittleEndian>()?)
    }
}

impl Decode for f64 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(reader.read_f64::<LittleEndian>()?)
    }
}

impl Decode for String {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let length = i32::decode(reader)?;

        if length > 0 {
            let mut characters = Vec::with_capacity((length - 1) as usize);

            for _ in 0..length - 1 {
                characters.push(char::decode(reader)?);
            }

            // Null terminator
            i32::decode(reader)?;

            Ok(characters.into_iter().collect::<String>())
        } else {
            Ok(String::new())
        }
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let is_available = bool::decode(reader)?;

        if is_available {
            Ok(Some(T::decode(reader)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Decode, const S: usize> Decode for [T; S] {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(array_init::try_array_init(|_| T::decode(reader))?)
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let length = i32::decode(reader)?;
        let mut elements = Vec::with_capacity(length as usize);

        for _ in 0..length {
            elements.push(T::decode(reader)?);
        }

        Ok(elements)
    }
}
