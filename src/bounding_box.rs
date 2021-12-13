use crate::{Decode, Vector3};
use std::io::Read;

pub struct BoundingBox {
    pub supremum: Vector3,
    pub infimum: Vector3,
}

impl BoundingBox {
    pub fn new(supremum: Vector3, infimum: Vector3) -> Self {
        Self { supremum, infimum }
    }
}

impl Decode for BoundingBox {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(Self::new(
            Vector3::decode(reader)?,
            Vector3::decode(reader)?,
        ))
    }
}
