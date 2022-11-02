use crate::{Decode, DecodeError, I32Encoded};
use std::io::Read;

#[derive(Clone, Debug, Default, Decode, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, Default, Decode, PartialEq, Eq, PartialOrd, Ord)]
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

impl Decode for I32Encoded<Rgba> {
    type Output = Rgba;

    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self::Output, DecodeError> {
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
