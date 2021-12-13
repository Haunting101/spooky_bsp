use std::{
    io::{self, Read},
    slice,
};

use crate::{hash, Rgba};
use byteorder::{LittleEndian, ReadBytesExt};

pub struct Texture {
    name: String,
    mask_name: String,
    width: i32,
    height: i32,
    filter: i32,
    address: i32,
    format: i32,
    border_color: Rgba,
    pixels: Vec<Rgba>,
}

impl Texture {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Texture> {
        let name_length = reader.read_i32::<LittleEndian>()?;
        let name = if name_length > 0 {
            let mut name = Vec::with_capacity((name_length - 1) as usize);

            for _ in 0..name_length - 1 {
                name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
            }

            // Null terminator
            reader.read_i32::<LittleEndian>()?;

            name.iter().collect::<String>()
        } else {
            String::new()
        };

        let mask_name_length = reader.read_i32::<LittleEndian>()?;
        let mask_name = if mask_name_length > 0 {
            let mut mask_name = Vec::with_capacity((mask_name_length - 1) as usize);

            for _ in 0..mask_name_length - 1 {
                mask_name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
            }

            // Null terminator
            reader.read_i32::<LittleEndian>()?;

            mask_name.into_iter().collect::<String>()
        } else {
            String::new()
        };

        let width = reader.read_i32::<LittleEndian>()?;
        let height = reader.read_i32::<LittleEndian>()?;
        let filter = reader.read_i32::<LittleEndian>()?;
        let address = reader.read_i32::<LittleEndian>()?;
        let format = reader.read_i32::<LittleEndian>()?;

        let border_color = Rgba::decode_i32(reader)?;

        let mut pixels = Vec::with_capacity((width * height) as usize);

        for _ in 0..width * height {
            pixels.push(Rgba::decode_i32(reader)?);
        }

        Ok(Texture {
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

    pub fn get_border_color(&self) -> &Rgba {
        &self.border_color
    }

    pub fn get_pixels(&self) -> &Vec<Rgba> {
        &self.pixels
    }

    pub fn get_hash(&self) -> u32 {
        let data = [self.address, self.filter];

        hash::hash(unsafe {
            slice::from_raw_parts(
                self.pixels.as_ptr() as *const _ as *const u8,
                (self.width * self.height * 4) as usize,
            )
        }) ^ hash::hash(unsafe { slice::from_raw_parts(data.as_ptr() as *const _ as *const u8, 8) })
    }
}
