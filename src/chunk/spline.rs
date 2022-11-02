use std::io::Read;

use crate::{Decode, DecodeError, Vector3};

#[derive(Clone, Debug)]
pub struct Spline {
    pub points: Vec<Vector3>,
    pub closed: bool,
    pub type_: u32,
}

impl Decode for Spline {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let points_count = u32::decode(reader, ())?;
        let closed = bool::decode(reader, ())?;
        let type_ = u32::decode(reader, ())?;
        let points = (0..points_count as usize)
            .into_iter()
            .map(|_| Vector3::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            points,
            closed,
            type_,
        })
    }
}
