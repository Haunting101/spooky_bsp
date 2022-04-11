use derive_new::new;
use std::io::Read;

use crate::{Decode, Rgba, Str};

#[derive(new)]
pub struct Texture {
    name: String,
    mask_name: String,
    width: i32,
    height: i32,
    filter: i32,
    address: i32,
    format: i32,
    border_color: Rgba<i32>,
    pixels: Vec<Rgba<i32>>,
}

impl Texture {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_mask_name(&self) -> &String {
        &self.mask_name
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_filter(&self) -> i32 {
        self.filter
    }

    pub fn get_address(&self) -> i32 {
        self.address
    }

    pub fn get_format(&self) -> i32 {
        self.format
    }

    pub fn get_border_color(&self) -> &Rgba<i32> {
        &self.border_color
    }

    pub fn get_pixels(&self) -> &Vec<Rgba<i32>> {
        &self.pixels
    }
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
