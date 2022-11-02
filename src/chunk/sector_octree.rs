use std::io::Read;

use crate::{BoundingBox, Decode, DecodeError};

#[derive(Clone, Debug, Decode)]
pub struct SectorOctree {
    pub blocks: Vec<SectorOctreeBlock>,
    pub leaves: Vec<SectorOctreeLeaf>,
    pub octants: Vec<SectorOctreeOctant>,
}

#[derive(Clone, Debug, Decode)]
pub struct SectorOctreeBlock {
    pub material_block_index: u32,
}

#[derive(Clone, Debug)]
pub struct SectorOctreeLeaf {
    pub sector_floor_flag: u32,
    pub world_blocks_count: i32,
    pub world_block_index: Option<u32>,
    pub zone_count: u32,
    pub zone: u32,
}

impl Decode for SectorOctreeLeaf {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let sector_floor_flag = u32::decode(reader, ())?;
        let world_blocks_count = i32::decode(reader, ())?;

        let world_block_index = if world_blocks_count > 0 {
            Some(u32::decode(reader, ())?)
        } else {
            None
        };

        let zone_count = u32::decode(reader, ())?;
        let zone = u32::decode(reader, ())?;

        Ok(Self {
            sector_floor_flag,
            world_blocks_count,
            world_block_index,
            zone_count,
            zone,
        })
    }
}

#[derive(Clone, Debug)]
pub enum SectorOctreeOctant {
    Leaf {
        bounds: BoundingBox,
        flags: u32,
        leaf_index: u32,
    },
    Subtree {
        bounds: BoundingBox,
        flags: u32,
        subtree_index: u32,
    },
}

impl Decode for SectorOctreeOctant {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let bounds = BoundingBox::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;
        let is_leaf = bool::decode(reader, ())?;

        if is_leaf {
            let leaf_index = u32::decode(reader, ())?;

            Ok(Self::Leaf {
                bounds,
                flags,
                leaf_index,
            })
        } else {
            let subtree_index = u32::decode(reader, ())?;

            Ok(Self::Subtree {
                bounds,
                flags,
                subtree_index,
            })
        }
    }
}
