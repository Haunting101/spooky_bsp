use crate::{Decode, QuantizedQuaternion, Vector3};

#[derive(Clone, Debug, Decode)]
pub struct AnimationDictionary {
    pub base_poses: Vec<BasePose>,
    pub clip_count: i32,
}

#[derive(Clone, Debug, Decode)]
pub struct BasePose {
    pub rotation: QuantizedQuaternion<i16>,
    pub position: Vector3,
}
