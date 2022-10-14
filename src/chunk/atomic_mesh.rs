use derive_new::new;
use std::io::Read;

use crate::Decode;

#[derive(new, Clone, Debug)]
pub struct AtomicMesh {
    pub base_flags: u32,
    pub flags: u32,
    pub name_hash: u32,
    pub has_mesh: bool,
}

impl Decode for AtomicMesh {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let base_flags = u32::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;
        let name_hash = u32::decode(reader, ())?;
        let has_mesh = bool::decode(reader, ())?;

        Ok(Self::new(base_flags, flags, name_hash, has_mesh))
    }
}
