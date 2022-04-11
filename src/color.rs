use crate::Decode;
use std::io::Read;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Rgb<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: Decode<Output = T>> Decode for Rgb<T> {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            T::decode(reader, ())?,
            T::decode(reader, ())?,
            T::decode(reader, ())?,
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: Decode<Output = T>> Decode for Rgba<T> {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            T::decode(reader, ())?,
            T::decode(reader, ())?,
            T::decode(reader, ())?,
            T::decode(reader, ())?,
        ))
    }
}

impl<T: Default> From<Rgb<T>> for Rgba<T> {
    fn from(rgb: Rgb<T>) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, T::default())
    }
}
