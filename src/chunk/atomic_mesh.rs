use crate::Decode;

#[derive(Clone, Debug, Decode)]
pub struct AtomicMesh {
    pub base_flags: u32,
    pub flags: u32,
    pub name_hash: u32,
    pub has_mesh: bool,
}
