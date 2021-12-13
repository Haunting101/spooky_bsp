use std::io::Read;

use crate::{Decode, Rgba, Vector3};

const HAS_VERTEX: u32 = 1 << 8;
const HAS_NORMAL: u32 = 1 << 9;
const HAS_RECIPROCAL_HOMOGENEOUS_W: u32 = 1 << 10;
const HAS_DIFFUSE: u32 = 1 << 11;
const HAS_WEIGHT: u32 = 1 << 12;
const HAS_INDICES: u32 = 1 << 13;
const UV_COUNT_MASK: u32 = 0xFF;

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
}

impl ModelPart {
    pub fn new(
        read_access_flags: u32,
        vertex_read_flags: u32,
        write_access_flags: u32,
        vertex_write_flags: u32,
        hint_flags: u32,
        constant_flags: u32,
        vertex_flags: u32,
        render_flags: u32,
        triangles_count: u16,
        strips_count: u16,
        strip_triangles_count: u16,
        material_hash: u32,
        triangle_index0: i32,
        triangle_index1: i32,
        vertex_index0: i32,
        vertex_index1: i32,
        layer_z: u32,
        floor_flags: u32,
        flags: u32,
        lighting_sid: u32,
        vertices: Vec<Vertex>,
    ) -> Self {
        Self {
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
        }
    }
}

impl Decode for ModelPart {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let read_access_flags = u32::decode(reader)?;
        let vertex_read_flags = u32::decode(reader)?;
        let write_access_flags = u32::decode(reader)?;
        let vertex_write_flags = u32::decode(reader)?;
        let hint_flags = u32::decode(reader)?;
        let constant_flags = u32::decode(reader)?;
        let vertex_flags = u32::decode(reader)?;
        let render_flags = u32::decode(reader)?;
        let vertex_count = u32::decode(reader)?;
        let triangles_count = u16::decode(reader)?;
        let strips_count = u16::decode(reader)?;
        let strip_triangles_count = u16::decode(reader)?;
        let material_hash = u32::decode(reader)?;
        let triangle_index0 = i32::decode(reader)?;
        let triangle_index1 = i32::decode(reader)?;
        let vertex_index0 = i32::decode(reader)?;
        let vertex_index1 = i32::decode(reader)?;
        let layer_z = u32::decode(reader)?;
        let floor_flags = u32::decode(reader)?;
        let flags = u32::decode(reader)?;
        let lighting_sid = u32::decode(reader)?;
        let vertices = (0..vertex_count)
            .into_iter()
            .map(|_| Vertex::decode(reader, flags))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(
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
        ))
    }
}

pub struct Vertex {
    pub vertex: Option<Vector3>,
    pub normal: Option<Vector3>,
    pub reciprocal_homogeneous_w: Option<u32>,
    pub diffuse: Option<Rgba<u8>>,
    pub weight: Option<f32>,
    pub indices: Option<(u16, u16)>,
    pub uvs: Vec<(f32, f32)>,
}

impl Vertex {
    pub fn new(
        vertex: Option<Vector3>,
        normal: Option<Vector3>,
        reciprocal_homogeneous_w: Option<u32>,
        diffuse: Option<Rgba<u8>>,
        weight: Option<f32>,
        indices: Option<(u16, u16)>,
        uvs: Vec<(f32, f32)>,
    ) -> Self {
        Self {
            vertex,
            normal,
            reciprocal_homogeneous_w,
            diffuse,
            weight,
            indices,
            uvs,
        }
    }

    pub fn decode(reader: &mut impl Read, flags: u32) -> eyre::Result<Self> {
        let vertex = if flags & HAS_VERTEX != 0 {
            let vertex = Vector3::decode(reader)?;

            Some(vertex)
        } else {
            None
        };

        let normal = if flags & HAS_NORMAL != 0 {
            let normal = Vector3::decode(reader)?;

            Some(normal)
        } else {
            None
        };

        let reciprocal_homogeneous_w = if flags & HAS_RECIPROCAL_HOMOGENEOUS_W != 0 {
            let reciprocal_homogeneous_w = u32::decode(reader)?;

            Some(reciprocal_homogeneous_w)
        } else {
            None
        };

        let diffuse = if flags & HAS_DIFFUSE != 0 {
            let diffuse = Rgba::<u8>::decode(reader)?;

            Some(diffuse)
        } else {
            None
        };

        let weight = if flags & HAS_WEIGHT != 0 {
            let weight = f32::decode(reader)?;

            Some(weight)
        } else {
            None
        };

        let indices = if flags & HAS_INDICES != 0 {
            let index0 = u16::decode(reader)?;
            let index1 = u16::decode(reader)?;

            Some((index0, index1))
        } else {
            None
        };

        let mut uvs = Vec::with_capacity((flags & UV_COUNT_MASK) as usize);

        for i in 0..flags & UV_COUNT_MASK {
            let u = f32::decode(reader)?;
            let v = f32::decode(reader)?;

            uvs.push((u, v));
        }

        Ok(Self::new(
            vertex,
            normal,
            reciprocal_homogeneous_w,
            diffuse,
            weight,
            indices,
            uvs,
        ))
    }
}
