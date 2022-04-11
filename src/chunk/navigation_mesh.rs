use crate::{Decode, Vector3};

use derive_new::new;
use std::io::Read;

#[derive(new)]
pub struct NavigationMesh {
    pub waypoints: Vec<Waypoint>,
    pub links: Vec<Link>,
}

impl Decode for NavigationMesh {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
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

        Ok(Self::new(waypoints, links))
    }
}

#[derive(new)]
pub struct Waypoint {
    pub position: Vector3,
    pub flags: u32,
}

impl Decode for Waypoint {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let position = Vector3::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;

        Ok(Waypoint::new(position, flags))
    }
}

pub struct Link {}

impl Decode for Link {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let mut waypoint_index = u32::decode(reader, ())?;
        let mut flags;

        while waypoint_index != u32::MAX {
            flags = u32::decode(reader, ())?;
            waypoint_index = u32::decode(reader, ())?;
        }

        Ok(Link {})
    }
}
