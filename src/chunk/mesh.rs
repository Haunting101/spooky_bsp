use crate::{BoundingBox, Decode, Vector3};

#[derive(Clone, Debug, Decode)]
pub struct Mesh {
    pub flags: u32,
    pub material_blocks_count: u16,
    pub bounds: BoundingBox,
    pub center: Vector3,
    pub radius: f32,
    pub have_bsp: bool,
}
