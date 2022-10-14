use derive_new::new;
use std::io::Read;

use crate::{Decode, Str};

#[derive(new, Clone, Debug)]
pub struct Clips {
    pub name_hash: u32,
    pub minimum_time: f32,
    pub maximum_time: f32,
    pub base_poses: Vec<Scaffold>,
    pub sequence_count: i32,
    pub name: String,
}

impl Decode for Clips {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let name_hash = u32::decode(reader, ())?;
        let minimum_time = f32::decode(reader, ())?;
        let maximum_time = f32::decode(reader, ())?;
        let base_pose_count = i32::decode(reader, ())?;
        let base_poses = (0..base_pose_count)
            .map(|_| Scaffold::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;
        let sequence_count = i32::decode(reader, ())?;
        let name = Str::<u8>::decode(reader, ())?;

        assert!(name.len() <= 32);

        Ok(Self::new(
            name_hash,
            minimum_time,
            maximum_time,
            base_poses,
            sequence_count,
            name,
        ))
    }
}

#[derive(new, Clone, Debug)]
pub struct Scaffold {
    pub hash1: u32,
    pub hash2: u32,
}

impl Decode for Scaffold {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            u32::decode(reader, ())?,
            u32::decode(reader, ())?,
        ))
    }
}
