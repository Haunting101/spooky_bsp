use std::io::{Read, self};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::BoundingBox;

pub struct SectorOctree {
    blocks: Vec<SectorOctreeBlock>,
    leaves: Vec<SectorOctreeLeaf>,
    octants: Vec<SectorOctreeOctant>,
}

impl SectorOctree {
    pub fn new(blocks: Vec<SectorOctreeBlock>, leaves: Vec<SectorOctreeLeaf>, octants: Vec<SectorOctreeOctant>) -> Self {
        Self {
            blocks,
            leaves,
            octants,
        }
    }

    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<SectorOctree> {
        let octree_blocks_count = reader.read_i32::<LittleEndian>()?;

        let mut blocks = Vec::with_capacity(octree_blocks_count as usize);

        for octree_block_index in 0 .. octree_blocks_count {
            let material_block_index = reader.read_u32::<LittleEndian>()?;

            blocks.push(SectorOctreeBlock::new(material_block_index));
        }

        let leaves_count = reader.read_i32::<LittleEndian>()?;

        let mut leaves = Vec::with_capacity(leaves_count as usize);

        for leaf_index in 0 .. leaves_count {
            let sector_floor_flag = reader.read_u32::<LittleEndian>()?;
            let world_blocks_count = reader.read_i32::<LittleEndian>()?;
            let zone_count = reader.read_u32::<LittleEndian>()?;
            let zone = reader.read_u32::<LittleEndian>()?;

            leaves.push(SectorOctreeLeaf::new(sector_floor_flag, world_blocks_count, zone_count, zone));
        }

        let octants_count = reader.read_i32::<LittleEndian>()?;

        let mut octants = Vec::with_capacity(octants_count as usize);

        for octant_index in 0 .. octants_count {
            let bounds = BoundingBox::decode(reader)?;
            let flags = reader.read_u32::<LittleEndian>()?;
            let is_leaf = reader.read_u32::<LittleEndian>()? != 0;

            if is_leaf {
                let leaf_index = reader.read_u32::<LittleEndian>()?;

                octants.push(SectorOctreeOctant::new_leaf(bounds, flags, leaf_index));
            } else {
                let subtree_index = reader.read_u32::<LittleEndian>()?;

                octants.push(SectorOctreeOctant::new_subtree(bounds, flags, subtree_index));
            }
        }

        Ok(SectorOctree::new(blocks, leaves, octants))
    }
}

pub struct SectorOctreeBlock {
    material_block_index: u32,
}

impl SectorOctreeBlock {
    pub fn new(material_block_index: u32) -> Self {
        Self {
            material_block_index,
        }
    }
}

pub struct SectorOctreeLeaf {
    sector_floor_flag: u32,
    world_blocks_count: i32,
    zone_count: u32,
    zone: u32,
}

impl SectorOctreeLeaf {
    pub fn new(sector_floor_flag: u32, world_blocks_count: i32, zone_count: u32, zone: u32) -> Self {
        Self {
            sector_floor_flag,
            world_blocks_count,
            zone_count,
            zone,
        }
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
