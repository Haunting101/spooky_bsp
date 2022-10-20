use derive_new::new;
use std::io::Read;

use crate::{Decode, Rgba, Str};

#[derive(new, Clone, Debug)]
pub struct Texture {
    pub name: String,
    pub mask_name: String,
    pub width: i32,
    pub height: i32,
    pub filter: i32,
    pub address: i32,
    pub format: i32,
    pub border_color: Rgba<i32>,
    pub pixels: Vec<Rgba<i32>>,
}

impl Decode for Texture {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Texture> {
        let name = Str::<i32, true>::decode(reader, ())?;
        let mask_name = Str::<i32, true>::decode(reader, ())?;
        let width = i32::decode(reader, ())?;
        let height = i32::decode(reader, ())?;
        let filter = i32::decode(reader, ())?;
        let address = i32::decode(reader, ())?;
        let format = i32::decode(reader, ())?;
        let border_color = Rgba::<i32>::decode(reader, ())?;
        let pixels = (0..width * height)
            .into_iter()
            .map(|_| Rgba::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Texture::new(
            name,
            mask_name,
            width,
            height,
            filter,
            address,
            format,
            border_color,
            pixels,
        ))
    }
}
