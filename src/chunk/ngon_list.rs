use std::io::Read;

use crate::{Decode, DecodeError, Plane, Vector3};

#[derive(Clone, Debug)]
pub struct NGonList {
    pub vertices: Vec<NGonVertex>,
    pub faces: Vec<NGonFace>,
}

impl Decode for NGonList {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let vertex_count = u32::decode(reader, ())?;
        let face_count = i32::decode(reader, ())?;

        let vertices = (0..vertex_count as usize)
            .map(|_| NGonVertex::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        let faces = (0..face_count as usize)
            .map(|_| NGonFace::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { vertices, faces })
    }
}

#[derive(Clone, Debug, Decode)]
pub struct NGonVertex {
    pub vector: Vector3,
    pub edge_plane: Plane,
}

#[derive(Clone, Debug)]
pub struct NGonFace {
    pub face_plane: Plane,
    pub vertex_index: u32,
    pub vertex_count: u32,
    pub flags: u32,
    pub test_count: u32,
}

impl Decode for NGonFace {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let face_plane = Plane::decode(reader, ())?;
        let vertex_index = u32::decode(reader, ())?;
        let vertex_count = u32::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;

        Ok(Self {
            face_plane,
            vertex_index,
            vertex_count,
            flags,
            test_count: 0,
        })
    }
}
