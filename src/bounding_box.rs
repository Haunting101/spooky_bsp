use crate::{Decode, Vector3};
use derive_new::new;
use std::io::Read;

#[derive(new, Clone, Debug)]
pub struct BoundingBox {
    pub supremum: Vector3,
    pub infimum: Vector3,
}

impl Decode for BoundingBox {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            Vector3::decode(reader, ())?,
            Vector3::decode(reader, ())?,
        ))
    }
}

#[derive(new, Clone, Debug)]
pub struct OrientedBoundingBox {
    pub center: Vector3,
    pub axes: [Vector3; 3],
    pub extents: [f32; 3],
}

impl Decode for OrientedBoundingBox {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let center = Vector3::decode(reader, ())?;
        let mut axes = [Vector3::default(); 3];
        let mut extents = [0.0; 3];

        for i in 0..=2 {
            axes[i] = Vector3::decode(reader, ())?;
            extents[i] = f32::decode(reader, ())?;
        }

        Ok(OrientedBoundingBox::new(center, axes, extents))
    }
}
