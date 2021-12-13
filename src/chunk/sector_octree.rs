use std::io::Read;

use crate::{BoundingBox, Decode};

pub struct SectorOctree {
    pub blocks: Vec<SectorOctreeBlock>,
    pub leaves: Vec<SectorOctreeLeaf>,
    pub octants: Vec<SectorOctreeOctant>,
}

impl SectorOctree {
    pub fn new(
        blocks: Vec<SectorOctreeBlock>,
        leaves: Vec<SectorOctreeLeaf>,
        octants: Vec<SectorOctreeOctant>,
    ) -> Self {
        Self {
            blocks,
            leaves,
            octants,
        }
    }
}

impl Decode for SectorOctree {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let blocks = Vec::decode(reader)?;
        let leaves = Vec::decode(reader)?;
        let octants = Vec::decode(reader)?;

        Ok(Self::new(blocks, leaves, octants))
    }
}

pub struct SectorOctreeBlock {
    pub material_block_index: u32,
}

impl SectorOctreeBlock {
    pub fn new(material_block_index: u32) -> Self {
        Self {
            material_block_index,
        }
    }
}

impl Decode for SectorOctreeBlock {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let material_block_index = u32::decode(reader)?;

        Ok(SectorOctreeBlock::new(material_block_index))
    }
}

pub struct SectorOctreeLeaf {
    pub sector_floor_flag: u32,
    pub world_blocks_count: i32,
    pub zone_count: u32,
    pub zone: u32,
}

impl SectorOctreeLeaf {
    pub fn new(
        sector_floor_flag: u32,
        world_blocks_count: i32,
        zone_count: u32,
        zone: u32,
    ) -> Self {
        Self {
            sector_floor_flag,
            world_blocks_count,
            zone_count,
            zone,
        }
    }
}

impl Decode for SectorOctreeLeaf {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let sector_floor_flag = u32::decode(reader)?;
        let world_blocks_count = i32::decode(reader)?;
        let zone_count = u32::decode(reader)?;
        let zone = u32::decode(reader)?;

        Ok(SectorOctreeLeaf::new(
            sector_floor_flag,
            world_blocks_count,
            zone_count,
            zone,
        ))
    }
}

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

impl SectorOctreeOctant {
    pub fn new_leaf(bounds: BoundingBox, flags: u32, leaf_index: u32) -> Self {
        Self::Leaf {
            bounds,
            flags,
            leaf_index,
        }
    }

    pub fn new_subtree(bounds: BoundingBox, flags: u32, subtree_index: u32) -> Self {
        Self::Subtree {
            bounds,
            flags,
            subtree_index,
        }
    }
}

impl Decode for SectorOctreeOctant {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let bounds = BoundingBox::decode(reader)?;
        let flags = u32::decode(reader)?;
        let is_leaf = bool::decode(reader)?;

        if is_leaf {
            let leaf_index = u32::decode(reader)?;

            Ok(SectorOctreeOctant::new_leaf(bounds, flags, leaf_index))
        } else {
            let subtree_index = u32::decode(reader)?;

            Ok(SectorOctreeOctant::new_subtree(
                bounds,
                flags,
                subtree_index,
            ))
        }
    }
}
