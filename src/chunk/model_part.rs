use std::io::Read;

use crate::{Decode, DecodeError, Rgba, Vector3};

const HAS_VERTEX: u32 = 1 << 8;
const HAS_RECIPROCAL_HOMOGENEOUS_W: u32 = 1 << 9;
const HAS_NORMAL: u32 = 1 << 10;
const HAS_DIFFUSE: u32 = 1 << 11;
const HAS_WEIGHT: u32 = 1 << 12;
const HAS_INDICES: u32 = 1 << 13;
const UV_COUNT_MASK: u32 = 0xFF;

#[derive(Clone, Debug)]
pub struct ModelPart {
    pub read_access_flags: u32,
    pub vertex_read_flags: u32,
    pub write_access_flags: u32,
    pub vertex_write_flags: u32,
    pub hint_flags: u32,
    pub constant_flags: u32,
    pub vertex_flags: u32,
    pub render_flags: u32,
    pub triangles_count: u16,
    pub strips_count: u16,
    pub strip_triangles_count: u16,
    pub material_hash: u32,
    pub triangle_index0: i32,
    pub triangle_index1: i32,
    pub vertex_index0: i32,
    pub vertex_index1: i32,
    pub layer_z: u32,
    pub floor_flags: u32,
    pub flags: u32,
    pub lighting_sid: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Decode for ModelPart {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let read_access_flags = u32::decode(reader, ())?;
        let vertex_read_flags = u32::decode(reader, ())?;
        let write_access_flags = u32::decode(reader, ())?;
        let vertex_write_flags = u32::decode(reader, ())?;
        let hint_flags = u32::decode(reader, ())?;
        let constant_flags = u32::decode(reader, ())?;
        let vertex_flags = u32::decode(reader, ())?;
        let render_flags = u32::decode(reader, ())?;
        let vertex_count = u32::decode(reader, ())?;
        let triangles_count = u16::decode(reader, ())?;
        let strips_count = u16::decode(reader, ())?;
        let strip_triangles_count = u16::decode(reader, ())?;

        let material_hash = u32::decode(reader, ())?;
        let triangle_index0 = i32::decode(reader, ())?;
        let triangle_index1 = i32::decode(reader, ())?;
        let vertex_index0 = i32::decode(reader, ())?;
        let vertex_index1 = i32::decode(reader, ())?;
        let layer_z = u32::decode(reader, ())?;

        let floor_flags = u32::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;
        let lighting_sid = u32::decode(reader, ())?;
        let vertices = (0..vertex_count)
            .into_iter()
            .map(|_| Vertex::decode(reader, vertex_flags))
            .collect::<Result<Vec<_>, _>>()?;
        let indices = (0..triangles_count)
            .into_iter()
            .map(|_| Index::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            read_access_flags,
            vertex_read_flags,
            write_access_flags,
            vertex_write_flags,
            hint_flags,
            constant_flags,
            vertex_flags,
            render_flags,
            triangles_count,
            strips_count,
            strip_triangles_count,
            material_hash,
            triangle_index0,
            triangle_index1,
            vertex_index0,
            vertex_index1,
            layer_z,
            floor_flags,
            flags,
            lighting_sid,
            vertices,
            indices,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub vertex: Option<Vector3>,
    pub normal: Option<Vector3>,
    pub reciprocal_homogeneous_w: Option<f32>,
    pub diffuse: Option<Rgba>,
    pub weight: Option<f32>,
    pub indices: Option<(u16, u16)>,
    pub uvs: Vec<(f32, f32)>,
}

impl Decode<u32> for Vertex {
    fn decode(reader: &mut impl Read, flags: u32) -> Result<Self, DecodeError> {
        let vertex = if flags & HAS_VERTEX != 0 {
            let vertex = Vector3::decode(reader, ())?;

            Some(vertex)
        } else {
            None
        };

        let normal = if flags & HAS_NORMAL != 0 {
            let normal = Vector3::decode(reader, ())?;

            Some(normal)
        } else {
            None
        };

        let reciprocal_homogeneous_w = if flags & HAS_RECIPROCAL_HOMOGENEOUS_W != 0 {
            let reciprocal_homogeneous_w = f32::decode(reader, ())?;

            Some(reciprocal_homogeneous_w)
        } else {
            None
        };

        let diffuse = if flags & HAS_DIFFUSE != 0 {
            let diffuse = Rgba::decode(reader, ())?;

            Some(diffuse)
        } else {
            None
        };

        let weight = if flags & HAS_WEIGHT != 0 {
            let weight = f32::decode(reader, ())?;

            Some(weight)
        } else {
            None
        };

        let indices = if flags & HAS_INDICES != 0 {
            let index0 = u16::decode(reader, ())?;
            let index1 = u16::decode(reader, ())?;

            Some((index0, index1))
        } else {
            None
        };

        let mut uvs = Vec::with_capacity((flags & UV_COUNT_MASK) as usize);

        for _ in 0..flags & UV_COUNT_MASK {
            let u = f32::decode(reader, ())?;
            let v = f32::decode(reader, ())?;

            uvs.push((u, v));
        }

        Ok(Self {
            vertex,
            normal,
            reciprocal_homogeneous_w,
            diffuse,
            weight,
            indices,
            uvs,
        })
    }
}

#[derive(Clone, Debug, Decode)]
pub struct Index {
    pub index0: u32,
    pub index1: u32,
    pub index2: u32,
}
