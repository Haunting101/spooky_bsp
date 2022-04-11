use crate::{BoundingBox, Decode, Matrix, Str};
use derive_new::new;
use std::io::Read;

pub type Nulls = Vec<Null>;

#[derive(new)]
pub struct Null {
    pub matrix: Matrix,
    pub bounding_box: BoundingBox,
    pub hash: u32,
    pub floor_flags: u32,
    pub flags: u32,
    pub spawn_type: u32,
    pub name: String,
}

impl Decode for Null {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let matrix = Matrix::decode(reader, ())?;
        let bounding_box = BoundingBox::decode(reader, ())?;
        let hash = u32::decode(reader, ())?;
        let floor_flags = u32::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;
        let spawn_type = u32::decode(reader, ())?;
        let name = Str::<u8>::decode(reader, ())?;

        Ok(Self::new(
            matrix,
            bounding_box,
            hash,
            floor_flags,
            flags,
            spawn_type,
            name,
        ))
    }
}
