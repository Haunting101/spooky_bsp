use crate::{Decode, DecodeError, QuantizedPlane};

use std::io::Read;

#[derive(Clone, Debug)]
pub struct Collision {
    pub faces: Vec<Leaf>,
    pub leaves: Vec<u32>,
    pub branches: Vec<Branch>,
}

impl Decode for Collision {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let face_count = u32::decode(reader, ())?;
        let leaf_count = u32::decode(reader, ())?;
        let branch_count = u32::decode(reader, ())?;

        let faces = (0..face_count)
            .into_iter()
            .map(|_| Leaf::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;
        let leaves = (0..leaf_count)
            .into_iter()
            .map(|_| u32::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;
        let branches = (0..branch_count)
            .into_iter()
            .map(|_| Branch::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            faces,
            leaves,
            branches,
        })
    }
}

#[derive(Clone, Debug, Decode)]
pub struct Leaf {
    pub plane: QuantizedPlane,
    pub material_block_index: u16,
    pub face_index: u16,
}

#[derive(Clone, Debug, Decode)]
pub struct Branch {
    pub plane: QuantizedPlane,
    pub index: u32,
}
