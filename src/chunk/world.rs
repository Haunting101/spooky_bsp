use std::io::Read;

use crate::{BoundingBox, Decode, DecodeError, Rgb, Rgba};

#[derive(Clone, Debug)]
pub struct World {
    pub flags: u32,
    pub ambient: Rgba,
    pub floors: Vec<Floor>,
    pub zone_count: i32,
    pub have_occlusion_bsp: bool,
    pub have_nulls: bool,
    pub have_waypoints: bool,
    pub have_mesh: bool,
}

impl Decode for World {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let flags = u32::decode(reader, ())?;
        let ambient = Rgb::decode(reader, ())?.into();
        let floors = Vec::decode(reader, ())?;
        let zone_count = i32::decode(reader, ())?;
        let have_occlusion_bsp = bool::decode(reader, ())?;
        let have_nulls = bool::decode(reader, ())?;
        let have_waypoints = bool::decode(reader, ())?;
        let have_mesh = bool::decode(reader, ())?;

        Ok(Self {
            flags,
            ambient,
            floors,
            zone_count,
            have_occlusion_bsp,
            have_nulls,
            have_waypoints,
            have_mesh,
        })
    }
}

#[derive(Clone, Debug, Decode)]
pub struct Floor {
    pub occlusion_bsp: u32,
    pub ghost_camera: BoundingBox,
}
