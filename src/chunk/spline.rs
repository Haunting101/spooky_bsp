use derive_new::new;
use std::io::Read;

use crate::{Decode, Vector3};

#[derive(new, Clone, Debug)]
pub struct Spline {
    pub points: Vec<Vector3>,
    pub closed: bool,
    pub type_: u32,
}

impl Decode for Spline {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let points_count = u32::decode(reader, ())?;
        let closed = bool::decode(reader, ())?;
        let type_ = u32::decode(reader, ())?;
        let points = (0..points_count as usize)
            .into_iter()
            .map(|_| Vector3::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(points, closed, type_))
    }
}
