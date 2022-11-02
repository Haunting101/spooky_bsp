use crate::{BoundingBox, Decode, Matrix, Vector3};

#[derive(Clone, Debug, Decode)]
pub struct Clump {
    pub base_flags: u32,
    pub name_hash: u32,
    pub flags: u64,
    pub floor_flags: u32,
    pub bone_count: Vec<Bone>,
    pub has_hierarchy: bool,
    pub default_animation_hash: u32,
    pub mirror_data: Option<MirrorData>,
}

#[derive(Clone, Debug, Decode)]
pub struct Bone {
    pub bone_id: u32,
    pub inverted_base_pose: Matrix,
}

#[derive(Clone, Debug, Decode)]
pub struct MirrorData {
    pub mirror_contents: BoundingBox,
    pub reflection_plane: ClumpPlane,
}

#[derive(Clone, Debug, Decode)]
pub struct ClumpPlane {
    pub normal: Vector3,
    pub point_on_plane: Vector3,
}
