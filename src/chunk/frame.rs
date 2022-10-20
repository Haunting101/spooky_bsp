use derive_new::new;
use std::io::Read;

use crate::{Decode, Matrix};

#[derive(new, Clone, Debug)]
pub struct Frame {
    pub local_transform_matrix: Matrix,
    pub global_transform_matrix: Matrix,
    pub bone_index: i32,
    pub flags: u32,
    pub id: u32,
    pub name: String,
}

impl Decode for Frame {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let local_transform_matrix = Matrix::decode(reader, ())?;
        let global_transform_matrix = Matrix::decode(reader, ())?;
        let bone_index = i32::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;
        let id = u32::decode(reader, ())?;
        let name = String::decode(reader, ())?;

        Ok(Self::new(
            local_transform_matrix,
            global_transform_matrix,
            bone_index,
            flags,
            id,
            name,
        ))
    }
}
