use byteorder::{LittleEndian, ReadBytesExt};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::{
    io::{self, Read},
    marker::PhantomData,
};

pub struct I32Encoded<T>(PhantomData<T>);
pub struct NullTerminated<T>(PhantomData<T>);

#[derive(Debug)]
pub enum DecodeError {
    ReadTooMuchData { expected: usize, actual: usize },
    ConversionFailure,
    IO(io::Error),
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

impl<E: TryFromPrimitive> From<TryFromPrimitiveError<E>> for DecodeError {
    fn from(_error: TryFromPrimitiveError<E>) -> Self {
        Self::ConversionFailure
    }
}

pub trait Decode<S = ()>
where
    Self: Sized,
{
    type Output = Self;

    fn decode(reader: &mut impl Read, state: S) -> Result<Self::Output, DecodeError>;
}

impl Decode for bool {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_i32::<LittleEndian>()? != 0)
    }
}

impl Decode for char {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(u8::decode(reader, ())? as char)
    }
}

impl Decode for i8 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_i8()?)
    }
}

impl Decode for u8 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_u8()?)
    }
}

impl Decode for i16 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_i16::<LittleEndian>()?)
    }
}

impl Decode for u16 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_u16::<LittleEndian>()?)
    }
}

impl Decode for i32 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_i32::<LittleEndian>()?)
    }
}

impl Decode for u32 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }
}

impl Decode for i64 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_i64::<LittleEndian>()?)
    }
}

impl Decode for u64 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }
}

impl Decode for f32 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_f32::<LittleEndian>()?)
    }
}

impl Decode for f64 {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(reader.read_f64::<LittleEndian>()?)
    }
}

impl Decode for String {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let length = i32::decode(reader, ())?;

        assert!(length >= 0);

        Ok((0..length)
            .into_iter()
            .map(|_| Ok(u8::decode(reader, ())? as char))
            .collect::<Result<String, DecodeError>>()?)
    }
}

impl Decode for I32Encoded<NullTerminated<String>> {
    type Output = String;

    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self::Output, DecodeError> {
        let length = i32::decode(reader, ())?;

        assert!(length >= 0);

        let string = (0..length - 1)
            .into_iter()
            .map(|_| Ok(i32::decode(reader, ())? as u8 as char))
            .collect::<Result<String, DecodeError>>()?;

        if length > 0 {
            let terminator = i32::decode(reader, ())?;

            assert_eq!(terminator, 0);
        }

        Ok(string)
    }
}

impl<S, T: Decode<S, Output = T>> Decode<S> for Option<T> {
    fn decode(reader: &mut impl Read, state: S) -> Result<Self::Output, DecodeError> {
        let is_available = bool::decode(reader, ())?;

        if is_available {
            Ok(Some(T::decode(reader, state)?))
        } else {
            Ok(None)
        }
    }
}

impl<S: Clone, T: Decode<S, Output = T>, const SIZE: usize> Decode<S> for [T; SIZE] {
    fn decode(reader: &mut impl Read, state: S) -> Result<Self::Output, DecodeError> {
        Ok(array_init::try_array_init(|_| {
            T::decode(reader, state.clone())
        })?)
    }
}

impl<S: Clone, T: Decode<S, Output = T>> Decode<S> for Vec<T> {
    fn decode(reader: &mut impl Read, state: S) -> Result<Self::Output, DecodeError> {
        let length = i32::decode(reader, ())?;

        assert!(length >= 0);

        let mut elements = Vec::with_capacity(length as usize);

        for _ in 0..length {
            elements.push(T::decode(reader, state.clone())?);
        }

        Ok(elements)
    }
}
