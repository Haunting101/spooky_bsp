use crate::{BoundingBox, Decode, Matrix};

pub type Nulls = Vec<Null>;

#[derive(Clone, Debug, Decode)]
pub struct Null {
    pub matrix: Matrix,
    pub bounding_box: BoundingBox,
    pub hash: u32,
    pub floor_flags: u32,
    pub flags: u32,
    pub spawn_type: u32,
    pub name: String,
}
