use std::io::Read;

use crate::{Decode, DecodeError, I32Encoded, NullTerminated, Rgba};

pub type Textures = Vec<Texture>;

#[derive(Clone, Debug)]
pub struct Texture {
    pub name: String,
    pub mask_name: String,
    pub width: i32,
    pub height: i32,
    pub filter: i32,
    pub address: i32,
    pub format: i32,
    pub border_color: Rgba,
    pub pixels: Vec<Rgba>,
}

impl Decode for Texture {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let name = I32Encoded::<NullTerminated<String>>::decode(reader, ())?;
        let mask_name = I32Encoded::<NullTerminated<String>>::decode(reader, ())?;
        let width = i32::decode(reader, ())?;
        let height = i32::decode(reader, ())?;
        let filter = i32::decode(reader, ())?;
        let address = i32::decode(reader, ())?;
        let format = i32::decode(reader, ())?;
        let border_color = I32Encoded::<Rgba>::decode(reader, ())?;
        let pixels = (0..width * height)
            .into_iter()
            .map(|_| I32Encoded::<Rgba>::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name,
            mask_name,
            width,
            height,
            filter,
            address,
            format,
            border_color,
            pixels,
        })
    }
}
