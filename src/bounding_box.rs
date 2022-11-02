use crate::{Decode, DecodeError, Vector3};
use std::io::Read;

#[derive(Clone, Debug, Decode)]
pub struct BoundingBox {
    pub supremum: Vector3,
    pub infimum: Vector3,
}

#[derive(Clone, Debug)]
pub struct OrientedBoundingBox {
    pub center: Vector3,
    pub axes: [Vector3; 3],
    pub extents: [f32; 3],
}

impl Decode for OrientedBoundingBox {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let center = Vector3::decode(reader, ())?;
        let mut axes = [Vector3::default(); 3];
        let mut extents = [0.0; 3];

        for i in 0..=2 {
            axes[i] = Vector3::decode(reader, ())?;
            extents[i] = f32::decode(reader, ())?;
        }

        Ok(OrientedBoundingBox {
            center,
            axes,
            extents,
        })
    }
}
