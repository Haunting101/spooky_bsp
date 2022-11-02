use crate::{Decode, OrientedBoundingBox};

#[derive(Clone, Debug, Decode)]
pub struct NullBox {
    pub base_flags: u32,
    pub null_index: u32,
    pub bounds: OrientedBoundingBox,
    pub name_hash: u32,
    pub spawn_type: u32,
}
