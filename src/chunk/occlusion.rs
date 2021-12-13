use crate::{Decode, Plane};
use std::io::Read;

pub struct Occlusion {
    pub branches: Vec<OcclusionBranch>,
    pub leaves: Vec<OcclusionLeaf>,
}

impl Occlusion {
    pub fn new(branches: Vec<OcclusionBranch>, leaves: Vec<OcclusionLeaf>) -> Self {
        Self { branches, leaves }
    }
}

impl Decode for Occlusion {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let is_plane_bsp = bool::decode(reader)?;
        let branches_count = u32::decode(reader)?;

        let mut branches = Vec::with_capacity(branches_count as usize);

        for branch_index in 0..branches_count {
            let plane = Plane::decode(reader)?;

            let negative_leaf;
            let negative;
            let positive_leaf;
            let positive;

            if is_plane_bsp {
                negative_leaf = u32::decode(reader)?;
                // TODO
                negative = 0;
                positive_leaf = u32::decode(reader)?;
                // TODO
                positive = 0;
            } else {
                negative_leaf = u32::decode(reader)?;
                negative = u32::decode(reader)?; // 32 bit void*
                positive_leaf = u32::decode(reader)?;
                positive = u32::decode(reader)?; // 32 bit void*
            }

            branches.push(OcclusionBranch::new(
                plane,
                negative_leaf,
                negative,
                positive_leaf,
                positive,
            ));
        }

        let leaves = Vec::decode(reader)?;

        Ok(Occlusion::new(branches, leaves))
    }
}

pub struct OcclusionBranch {
    pub plane: Plane,
    pub negative_leaf: u32,
    pub negative: u32,
    pub positive_leaf: u32,
    pub positive: u32,
}

impl OcclusionBranch {
    pub fn new(
        plane: Plane,
        negative_leaf: u32,
        negative: u32,
        positive_leaf: u32,
        positive: u32,
    ) -> Self {
        Self {
            plane,
            negative_leaf,
            negative,
            positive_leaf,
            positive,
        }
    }
}

pub struct OcclusionLeaf {
    pub faces: u32,
    pub have_occlusion_meshes: bool,
}

impl OcclusionLeaf {
    pub fn new(faces: u32, have_occlusion_meshes: bool) -> Self {
        Self {
            faces,
            have_occlusion_meshes,
        }
    }
}

impl Decode for OcclusionLeaf {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(OcclusionLeaf::new(
            u32::decode(reader)?,
            bool::decode(reader)?,
        ))
    }
}
