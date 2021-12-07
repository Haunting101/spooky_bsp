use std::io::{Read, self};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{BoundingBox, Vector3};

pub struct Mesh {
    flags: u32,
    material_blocks_count: u16,
    bounds: BoundingBox,
    center: Vector3,
    radius: f32,
    have_bsp: bool,
}

impl Mesh {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Mesh> {
        let flags = reader.read_u32::<LittleEndian>()?;
        let material_blocks_count = reader.read_u16::<LittleEndian>()?;
        let bounds = BoundingBox::decode(reader)?;
        let center = Vector3::decode(reader)?;
        let radius = reader.read_f32::<LittleEndian>()?;
        let have_bsp = reader.read_i32::<LittleEndian>()? != 0;

        Ok(Mesh { flags, material_blocks_count, bounds, center, radius, have_bsp })
    }
}
