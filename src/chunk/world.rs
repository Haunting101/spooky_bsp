use derive_new::new;
use std::io::Read;

use crate::{BoundingBox, Decode, Rgb, Rgba};

#[derive(new, Clone, Debug)]
pub struct World {
    pub flags: u32,
    pub ambient: Rgba<u8>,
    pub floors: Vec<Floor>,
    pub zone_count: i32,
    pub have_occlusion_bsp: bool,
    pub have_nulls: bool,
    pub have_waypoints: bool,
    pub have_mesh: bool,
}

impl Decode for World {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let flags = u32::decode(reader, ())?;
        let ambient = Rgb::<u8>::decode(reader, ())?.into();
        let floors = Vec::decode(reader, ())?;
        let zone_count = i32::decode(reader, ())?;
        let have_occlusion_bsp = bool::decode(reader, ())?;
        let have_nulls = bool::decode(reader, ())?;
        let have_waypoints = bool::decode(reader, ())?;
        let have_mesh = bool::decode(reader, ())?;

        Ok(Self::new(
            flags,
            ambient,
            floors,
            zone_count,
            have_occlusion_bsp,
            have_nulls,
            have_waypoints,
            have_mesh,
        ))
    }
}

#[derive(new, Clone, Debug)]
pub struct Floor {
    pub occlusion_bsp: u32,
    pub ghost_camera: BoundingBox,
}

impl Decode for Floor {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let occlusion_bsp = u32::decode(reader, ())?;
        let ghost_camera = BoundingBox::decode(reader, ())?;

        Ok(Self::new(occlusion_bsp, ghost_camera))
    }
}
