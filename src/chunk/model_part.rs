use std::io::{Read, self};

use byteorder::{ReadBytesExt, LittleEndian};

use crate::{Vector3, Rgba};

const HAS_VERTEX: u32 = 1 << 8;
const HAS_NORMAL: u32 = 1 << 9;
const HAS_RECIPROCAL_HOMOGENEOUS_W: u32 = 1 << 10;
const HAS_DIFFUSE: u32 = 1 << 11;
const HAS_WEIGHT: u32 = 1 << 12;
const HAS_INDICES: u32 = 1 << 13;
const UV_COUNT_MASK: u32 = 0xFF;

pub struct ModelPart {
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
}

impl ModelPart {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Self> {
        let read_access_flags = reader.read_u32::<LittleEndian>()?;
        let vertex_read_flags = reader.read_u32::<LittleEndian>()?;
        let write_access_flags = reader.read_u32::<LittleEndian>()?;
        let vertex_write_flags = reader.read_u32::<LittleEndian>()?;
        let hint_flags = reader.read_u32::<LittleEndian>()?;
        let constant_flags = reader.read_u32::<LittleEndian>()?;
        let vertex_flags = reader.read_u32::<LittleEndian>()?;
        let render_flags = reader.read_u32::<LittleEndian>()?;
        let vertex_count = reader.read_u32::<LittleEndian>()?;
        let triangles_count = reader.read_u16::<LittleEndian>()?;
        let strips_count = reader.read_u16::<LittleEndian>()?;
        let strip_triangles_count = reader.read_u16::<LittleEndian>()?;
        let material_hash = reader.read_u32::<LittleEndian>()?;
        let triangle_index0 = reader.read_i32::<LittleEndian>()?;
        let triangle_index1 = reader.read_i32::<LittleEndian>()?;
        let vertex_index0 = reader.read_i32::<LittleEndian>()?;
        let vertex_index1 = reader.read_i32::<LittleEndian>()?;
        let layer_z = reader.read_u32::<LittleEndian>()?;
        let floor_flags = reader.read_u32::<LittleEndian>()?;
        let flags = reader.read_u32::<LittleEndian>()?;
        let lighting_sid = reader.read_u32::<LittleEndian>()?;

        let mut vertices = Vec::with_capacity(vertex_count as usize);

        for vertex_index in 0 .. vertex_count {
            let vertex = Vertex::decode(reader, flags)?;

            vertices.push(vertex);
        }

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
        })
    }
}

pub struct Vertex {
    vertex: Option<Vector3>,
    normal: Option<Vector3>,
    reciprocal_homogeneous_w: Option<u32>,
    diffuse: Option<Rgba>,
    weight: Option<f32>,
    indices: Option<(u16, u16)>,
    uvs: Vec<(f32, f32)>,
}

impl Vertex {
    pub(crate) fn decode(reader: &mut impl Read, flags: u32) -> io::Result<Self> {
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
            let reciprocal_homogeneous_w = reader.read_u32::<LittleEndian>()?;

            Some(reciprocal_homogeneous_w)
        } else {
            None
        };

        let diffuse = if flags & HAS_DIFFUSE != 0 {
            let diffuse = Rgba::decode_u8(reader)?;

            Some(diffuse)
        } else {
            None
        };

        let weight = if flags & HAS_WEIGHT != 0 {
            let weight = reader.read_f32::<LittleEndian>()?;

            Some(weight)
        } else {
            None
        };

        let indices = if flags & HAS_INDICES != 0 {
            let index0 = reader.read_u16::<LittleEndian>()?;
            let index1 = reader.read_u16::<LittleEndian>()?;

            Some((index0, index1))
        } else {
            None
        };

        let mut uvs = Vec::with_capacity((flags & UV_COUNT_MASK) as usize);

        for i in 0 .. flags & UV_COUNT_MASK {
            let u = reader.read_f32::<LittleEndian>()?;
            let v = reader.read_f32::<LittleEndian>()?;

            uvs.push((u, v));
        }

        Ok(Self { vertex, normal, reciprocal_homogeneous_w, diffuse, weight, indices, uvs })
    }
}
