use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

pub trait Decode<S = ()>
where
    Self: Sized,
{
    type Output = Self;

    fn decode(reader: &mut impl Read, state: S) -> eyre::Result<Self::Output>;
}

impl Decode for bool {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_i32::<LittleEndian>()? != 0)
    }
}

impl Decode for char {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(u8::decode(reader, ())? as char)
    }
}

impl Decode for i8 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_i8()?)
    }
}

impl Decode for u8 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_u8()?)
    }
}

impl Decode for i16 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_i16::<LittleEndian>()?)
    }
}

impl Decode for u16 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_u16::<LittleEndian>()?)
    }
}

impl Decode for i32 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_i32::<LittleEndian>()?)
    }
}

impl Decode for u32 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }
}

impl Decode for i64 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_i64::<LittleEndian>()?)
    }
}

impl Decode for u64 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }
}

impl Decode for f32 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_f32::<LittleEndian>()?)
    }
}

impl Decode for f64 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(reader.read_f64::<LittleEndian>()?)
    }
}

impl<S, T: Decode<S, Output = T>> Decode<S> for Option<T> {
    fn decode(reader: &mut impl Read, state: S) -> eyre::Result<Self::Output> {
        let is_available = bool::decode(reader, ())?;

        if is_available {
            Ok(Some(T::decode(reader, state)?))
        } else {
            Ok(None)
        }
    }
}

impl<S: Clone, T: Decode<S, Output = T>, const SIZE: usize> Decode<S> for [T; SIZE] {
    fn decode(reader: &mut impl Read, state: S) -> eyre::Result<Self::Output> {
        Ok(array_init::try_array_init(|_| {
            T::decode(reader, state.clone())
        })?)
    }
}

impl<S: Clone, T: Decode<S, Output = T>> Decode<S> for Vec<T> {
    fn decode(reader: &mut impl Read, state: S) -> eyre::Result<Self::Output> {
        let length = i32::decode(reader, ())?;

        assert!(length >= 0);

        let mut elements = Vec::with_capacity(length as usize);

        for _ in 0..length {
            elements.push(T::decode(reader, state.clone())?);
        }

        Ok(elements)
    }
}
