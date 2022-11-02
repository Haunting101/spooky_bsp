use crate::{Decode, Rectangle};

#[derive(Clone, Debug, Decode)]
pub struct CameraProjection {
    pub type_: i32,
    pub near_z: f32,
    pub far_z: f32,
    pub angle_y: f32,
    pub rectangle: Rectangle,
}
