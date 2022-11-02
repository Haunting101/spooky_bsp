use crate::Decode;

#[derive(Clone, Debug, Decode)]
pub struct Clips {
    pub name_hash: u32,
    pub minimum_time: f32,
    pub maximum_time: f32,
    pub base_poses: Vec<Scaffold>,
    pub sequence_count: i32,
    pub name: String,
}

#[derive(Clone, Debug, Decode)]
pub struct Scaffold {
    pub hash1: u32,
    pub hash2: u32,
}
