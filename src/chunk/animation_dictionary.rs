use derive_new::new;
use std::io::Read;

use crate::{Decode, QuantizedQuaternion, Vector3};

#[derive(new)]
pub struct AnimationDictionary {
    pub base_poses: Vec<BasePose>,
    pub clip_count: i32,
}

impl Decode for AnimationDictionary {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let base_pose_count = i32::decode(reader, ())?;
        let base_poses = (0..base_pose_count)
            .map(|_| BasePose::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;
        let clip_count = i32::decode(reader, ())?;

        Ok(Self::new(base_poses, clip_count))
    }
}

#[derive(new)]
pub struct BasePose {
    pub rotation: QuantizedQuaternion<i16>,
    pub position: Vector3,
}

impl Decode for BasePose {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            QuantizedQuaternion::decode(reader, ())?,
            Vector3::decode(reader, ())?,
        ))
    }
}
