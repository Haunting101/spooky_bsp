use crate::{Decode, QuantizedPlane};

use derive_new::new;
use std::io::Read;

#[derive(new, Clone, Debug)]
pub struct Collision {
    pub faces: Vec<Leaf>,
    pub leaves: Vec<u32>,
    pub branches: Vec<Branch>,
}

impl Decode for Collision {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
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

        Ok(Self::new(faces, leaves, branches))
    }
}

#[derive(new, Clone, Debug)]
pub struct Leaf {
    pub plane: QuantizedPlane,
    pub material_block_index: u16,
    pub face_index: u16,
}

impl Decode for Leaf {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let plane = QuantizedPlane::decode(reader, ())?;
        let material_block_index = u16::decode(reader, ())?;
        let face_index = u16::decode(reader, ())?;

        Ok(Self::new(plane, material_block_index, face_index))
    }
}

#[derive(new, Clone, Debug)]
pub struct Branch {
    pub plane: QuantizedPlane,
    pub index: u32,
}

impl Decode for Branch {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let plane = QuantizedPlane::decode(reader, ())?;
        let index = u32::decode(reader, ())?;

        Ok(Self::new(plane, index))
    }
}
