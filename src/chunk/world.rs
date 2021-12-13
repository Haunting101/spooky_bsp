use std::io::Read;

use crate::{BoundingBox, Decode, Rgb, Rgba};

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

impl World {
    pub fn new(
        flags: u32,
        ambient: Rgba<u8>,
        floors: Vec<Floor>,
        zone_count: i32,
        have_occlusion_bsp: bool,
        have_nulls: bool,
        have_waypoints: bool,
        have_mesh: bool,
    ) -> Self {
        Self {
            flags,
            ambient,
            floors,
            zone_count,
            have_occlusion_bsp,
            have_nulls,
            have_waypoints,
            have_mesh,
        }
    }
}

impl Decode for World {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let flags = u32::decode(reader)?;
        let ambient = Rgb::<u8>::decode(reader)?.into();
        let floors = Vec::decode(reader)?;
        let zone_count = i32::decode(reader)?;
        let have_occlusion_bsp = bool::decode(reader)?;
        let have_nulls = bool::decode(reader)?;
        let have_waypoints = bool::decode(reader)?;
        let have_mesh = bool::decode(reader)?;

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

pub struct Floor {
    pub occlusion_bsp: u32,
    pub ghost_camera: BoundingBox,
}

impl Floor {
    pub fn new(occlusion_bsp: u32, ghost_camera: BoundingBox) -> Self {
        Self {
            occlusion_bsp,
            ghost_camera,
        }
    }
}

impl Decode for Floor {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let occlusion_bsp = u32::decode(reader)?;
        let ghost_camera = BoundingBox::decode(reader)?;

        Ok(Self::new(occlusion_bsp, ghost_camera))
    }
}
