use crate::{Decode, DecodeError, Plane};
use std::io::Read;

#[derive(Clone, Debug)]
pub struct Occlusion {
    pub branches: Vec<OcclusionBranch>,
    pub leaves: Vec<OcclusionLeaf>,
    pub has_occlusion_meshes: bool,
}

impl Decode for Occlusion {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let is_plane_bsp = bool::decode(reader, ())?;
        let branches_count = u32::decode(reader, ())?;
        let branches = (0..branches_count)
            .into_iter()
            .map(|_| OcclusionBranch::decode(reader, is_plane_bsp))
            .collect::<Result<Vec<_>, _>>()?;
        let leaves = Vec::decode(reader, ())?;
        let has_occlusion_meshes = bool::decode(reader, ())?;

        Ok(Self {
            branches,
            leaves,
            has_occlusion_meshes,
        })
    }
}

#[derive(Clone, Debug)]
pub struct OcclusionBranch {
    pub plane: Plane,
    pub negative_leaf: u32,
    pub negative: u32,
    pub positive_leaf: u32,
    pub positive: u32,
}

impl Decode<bool> for OcclusionBranch {
    fn decode(reader: &mut impl Read, is_plane_bsp: bool) -> Result<Self, DecodeError> {
        let plane = Plane::decode(reader, ())?;

        let negative_leaf = u32::decode(reader, ())?;
        let negative = if is_plane_bsp {
            0
        } else {
            u32::decode(reader, ())?
        };
        let positive_leaf = u32::decode(reader, ())?;
        let positive = if is_plane_bsp {
            0
        } else {
            u32::decode(reader, ())?
        };

        Ok(Self {
            plane,
            negative_leaf,
            negative,
            positive_leaf,
            positive,
        })
    }
}

#[derive(Clone, Debug, Decode)]
pub struct OcclusionLeaf {
    pub faces: u32,
}
