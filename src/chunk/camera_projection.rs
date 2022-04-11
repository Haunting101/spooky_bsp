use derive_new::new;
use std::io::Read;

use crate::{Decode, Rectangle};

#[derive(new)]
pub struct CameraProjection {
    pub type_: i32,
    pub near_z: f32,
    pub far_z: f32,
    pub angle_y: f32,
    pub rectangle: Rectangle,
}

impl Decode for CameraProjection {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self {
            type_: i32::decode(reader, ())?,
            near_z: f32::decode(reader, ())?,
            far_z: f32::decode(reader, ())?,
            angle_y: f32::decode(reader, ())?,
            rectangle: Rectangle::decode(reader, ())?,
        })
    }
}
