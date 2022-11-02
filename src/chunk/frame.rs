use crate::{Decode, Matrix};

#[derive(Clone, Debug, Decode)]
pub struct Frame {
    pub local_transform_matrix: Matrix,
    pub global_transform_matrix: Matrix,
    pub bone_index: i32,
    pub flags: u32,
    pub id: u32,
    pub name: String,
}
