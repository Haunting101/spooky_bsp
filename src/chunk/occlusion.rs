use std::io::{Read, self};

use byteorder::{LittleEndian, ReadBytesExt};

pub struct Occlusion {
    branches: Vec<OcclusionBranch>,
    leaves: Vec<OcclusionLeaf>,
}

impl Occlusion {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Occlusion> {
        let is_plane_bsp = reader.read_i32::<LittleEndian>()? != 0;
        let branches_count = reader.read_u32::<LittleEndian>()?;

        let mut branches = Vec::with_capacity(branches_count as usize);

        for branch_index in 0 .. branches_count {
            let plane = Plane::decode(reader)?;

            let negative_leaf;
            let negative;
            let positive_leaf;
            let positive;

            if is_plane_bsp {
                negative_leaf = reader.read_u32::<LittleEndian>()?;
                // TODO
                negative = 0;
                positive_leaf = reader.read_u32::<LittleEndian>()?;
                // TODO
                positive = 0;
            } else {
                negative_leaf = reader.read_u32::<LittleEndian>()?;
                negative = reader.read_u32::<LittleEndian>()?; // 32 bit void*
                positive_leaf = reader.read_u32::<LittleEndian>()?;
                positive = reader.read_u32::<LittleEndian>()?; // 32 bit void*
            }

            branches.push(OcclusionBranch::new(plane, negative_leaf, negative, positive_leaf, positive));
        }

        let leaf_count = reader.read_u32::<LittleEndian>()?;

        let mut leaves = Vec::with_capacity(leaf_count as usize);

        for leaf_index in 0 .. leaf_count {
            let faces = reader.read_u32::<LittleEndian>()?;
            let have_occlusion_meshes = reader.read_i32::<LittleEndian>()? != 0;

            leaves.push(OcclusionLeaf::new(faces, have_occlusion_meshes));
        }

        Ok(Occlusion {
            branches,
            leaves,
        })
    }
}

pub struct OcclusionBranch {
    plane: Plane,
    negative_leaf: u32,
    negative: u32,
    positive_leaf: u32,
    positive: u32,
}

impl OcclusionBranch {
    pub fn new(plane: Plane, negative_leaf: u32, negative: u32, positive_leaf: u32, positive: u32) -> Self {
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
    faces: u32,
    have_occlusion_meshes: bool,
}

impl OcclusionLeaf {
    pub fn new(faces: u32, have_occlusion_meshes: bool) -> Self {
        Self {
            faces,
            have_occlusion_meshes,
        }
    }
}

// TODO
pub struct Plane {

}

impl Plane {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Plane> {
        reader.read_f32::<LittleEndian>()?;
        reader.read_f32::<LittleEndian>()?;
        reader.read_f32::<LittleEndian>()?;
        reader.read_f32::<LittleEndian>()?;

        Ok(Plane {})
    }
}