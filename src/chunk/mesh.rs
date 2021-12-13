use std::io::Read;

use crate::{BoundingBox, Decode, Vector3};

pub struct Mesh {
    pub flags: u32,
    pub material_blocks_count: u16,
    pub bounds: BoundingBox,
    pub center: Vector3,
    pub radius: f32,
    pub have_bsp: bool,
}

impl Mesh {
    pub fn new(
        flags: u32,
        material_blocks_count: u16,
        bounds: BoundingBox,
        center: Vector3,
        radius: f32,
        have_bsp: bool,
    ) -> Self {
        Self {
            flags,
            material_blocks_count,
            bounds,
            center,
            radius,
            have_bsp,
        }
    }
}

impl Decode for Mesh {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(Self::new(
            u32::decode(reader)?,
            u16::decode(reader)?,
            BoundingBox::decode(reader)?,
            Vector3::decode(reader)?,
            f32::decode(reader)?,
            bool::decode(reader)?,
        ))
    }
}
