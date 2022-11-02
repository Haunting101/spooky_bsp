use crate::{Decode, DecodeError, Vector3};

use std::io::Read;

#[derive(Clone, Debug)]
pub struct NavigationMesh {
    pub waypoints: Vec<Waypoint>,
    pub links: Vec<Link>,
}

impl Decode for NavigationMesh {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let waypoint_count = i32::decode(reader, ())?;

        assert!(waypoint_count >= 0);

        let link_count = i32::decode(reader, ())?;

        assert!(link_count >= 0);

        let waypoints = (0..waypoint_count)
            .into_iter()
            .map(|_| Waypoint::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;
        let links = (0..waypoint_count)
            .into_iter()
            .map(|_| Link::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { waypoints, links })
    }
}

#[derive(Clone, Debug, Decode)]
pub struct Waypoint {
    pub position: Vector3,
    pub flags: u32,
}

#[derive(Clone, Debug)]
pub struct Link {
    pub waypoint_index: u32,
    pub flags: u32,
}

impl Decode for Link {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let mut waypoint_index = u32::decode(reader, ())?;
        let mut flags = 0;

        while waypoint_index != u32::MAX {
            flags = u32::decode(reader, ())?;
            waypoint_index = u32::decode(reader, ())?;
        }

        Ok(Self {
            waypoint_index,
            flags,
        })
    }
}
