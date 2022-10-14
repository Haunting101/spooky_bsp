use derive_new::new;
use std::io::Read;

use crate::{Decode, OrientedBoundingBox};

#[derive(new, Clone, Debug)]
pub struct NullBox {
    pub base_flags: u32,
    pub null_index: u32,
    pub bounds: OrientedBoundingBox,
    pub name_hash: u32,
    pub spawn_type: u32,
}

impl Decode for NullBox {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let base_flags = u32::decode(reader, ())?;
        let null_index = u32::decode(reader, ())?;
        let bounds = OrientedBoundingBox::decode(reader, ())?;
        let name_hash = u32::decode(reader, ())?;
        let spawn_type = u32::decode(reader, ())?;

        Ok(Self::new(
            base_flags, null_index, bounds, name_hash, spawn_type,
        ))
    }
}
