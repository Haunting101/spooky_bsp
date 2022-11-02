use crate::{BoundingBox, ChunkHeader, Decode, DecodeError, World};

use std::io::Read;

#[derive(Clone, Debug)]
pub struct Zones {
    pub octant_connections: Vec<u32>,
    pub zones: Vec<Zone>,
}

impl Decode<(&ChunkHeader, &World)> for Zones {
    fn decode(
        reader: &mut impl Read,
        (header, world): (&ChunkHeader, &World),
    ) -> Result<Self, DecodeError> {
        let octant_connections = Vec::decode(reader, ())?;
        let zones = (0..world.zone_count)
            .into_iter()
            .map(|_| Zone::decode(reader, header))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            octant_connections,
            zones,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Zone {
    pub bounding_box: BoundingBox,
    pub hash: u32,
    pub ngon_index: u32,
    pub spline_index: u32,
    pub clump_index: u32,
    pub floor_flags: u32,
    pub zone_top: Option<u32>,
}

impl Decode<&ChunkHeader> for Zone {
    fn decode(reader: &mut impl Read, header: &ChunkHeader) -> Result<Self, DecodeError> {
        let bounding_box = BoundingBox::decode(reader, ())?;
        let hash = u32::decode(reader, ())?;
        let ngon_index = u32::decode(reader, ())?;
        let spline_index = u32::decode(reader, ())?;
        let clump_index = u32::decode(reader, ())?;
        let floor_flags = u32::decode(reader, ())?;

        let zone_top = if header.get_version() >= 0x666 + 0x3C {
            Some(u32::decode(reader, ())?)
        } else {
            None
        };

        Ok(Self {
            bounding_box,
            hash,
            ngon_index,
            spline_index,
            clump_index,
            floor_flags,
            zone_top,
        })
    }
}
