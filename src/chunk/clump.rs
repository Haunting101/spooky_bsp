use derive_new::new;
use std::io::Read;

use crate::{BoundingBox, Decode, Matrix, Vector3};

#[derive(new)]
pub struct Clump {
    pub base_flags: u32,
    pub name_hash: u32,
    pub flags: u64,
    pub floor_flags: u32,
    pub bone_count: u32,
}

impl Decode for Clump {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let base_flags = u32::decode(reader, ())?;
        let name_hash = u32::decode(reader, ())?;
        let flags = u64::decode(reader, ())?;
        let floor_flags = u32::decode(reader, ())?;
        let bone_count = u32::decode(reader, ())?;

        if bone_count > 0 {
            let bone_ids = (0..bone_count)
                .map(|_| u32::decode(reader, ()))
                .collect::<Result<Vec<_>, _>>()?;
            let inverted_base_poses = (0..bone_count)
                .map(|_| Matrix::decode(reader, ()))
                .collect::<Result<Vec<_>, _>>()?;
        }

        let has_hierarchy = bool::decode(reader, ())?;
        let default_animation_hash = u32::decode(reader, ())?;
        let has_mirror_data = bool::decode(reader, ())?;

        if has_mirror_data {
            let mirror_contents = BoundingBox::decode(reader, ())?;
            let reflection_plane = Plane::decode(reader, ())?;
        }

        Ok(Self::new(
            base_flags,
            name_hash,
            flags,
            floor_flags,
            bone_count,
        ))
    }
}

#[derive(new)]
struct Plane {
    pub normal: Vector3,
    pub point_on_plane: Vector3,
}

impl Decode for Plane {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            Vector3::decode(reader, ())?,
            Vector3::decode(reader, ())?,
        ))
    }
}
