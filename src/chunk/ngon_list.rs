use derive_new::new;
use std::io::Read;

use crate::{Decode, Plane, Vector3};

#[derive(new)]
pub struct NGonList {
    pub vertices: Vec<NGonVertex>,
    pub faces: Vec<NGonFace>,
}

impl Decode for NGonList {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let vertex_count = u32::decode(reader, ())?;
        let face_count = i32::decode(reader, ())?;

        let vertices = (0..vertex_count as usize)
            .map(|_| NGonVertex::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        let faces = (0..face_count as usize)
            .map(|_| NGonFace::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(vertices, faces))
    }
}

#[derive(new)]
pub struct NGonVertex {
    pub vector: Vector3,
    pub edge_plane: Plane,
}

impl Decode for NGonVertex {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let vector = Vector3::decode(reader, ())?;
        let edge_plane = Plane::decode(reader, ())?;

        Ok(Self::new(vector, edge_plane))
    }
}

#[derive(new)]
pub struct NGonFace {
    pub face_plane: Plane,
    pub vertex_index: u32,
    pub vertex_count: u32,
    pub flags: u32,
    pub test_count: u32,
}

impl Decode for NGonFace {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let face_plane = Plane::decode(reader, ())?;
        let vertex_index = u32::decode(reader, ())?;
        let vertex_count = u32::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;

        Ok(Self::new(face_plane, vertex_index, vertex_count, flags, 0))
    }
}
