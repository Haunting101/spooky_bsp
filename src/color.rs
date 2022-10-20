use crate::{Decode, I32Encoded};
use std::io::Read;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Decode for Rgb {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Decode for Rgba {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
        ))
    }
}

impl Decode for I32Encoded<Rgba> {
    type Output = Rgba;

    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self::Output> {
        Ok(Rgba::new(
            i32::decode(reader, ())? as u8,
            i32::decode(reader, ())? as u8,
            i32::decode(reader, ())? as u8,
            i32::decode(reader, ())? as u8,
        ))
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, u8::default())
    }
}
