use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Matrix {
    pub right: Vector4,
    pub up: Vector4,
    pub at: Vector4,
    pub position: Vector4,
    pub flags: u64,
}

impl Matrix {
    pub fn new(right: Vector4, up: Vector4, at: Vector4, position: Vector4, flags: u64) -> Self {
        Self {
            right,
            up,
            at,
            position,
            flags,
        }
    }

    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Self> {
        let right = Vector3::decode(reader)?.into();
        let up = Vector3::decode(reader)?.into();
        let at = Vector3::decode(reader)?.into();
        let position = Vector3::decode(reader)?;
        let position = Vector4::new(position.x, position.y, position.z, 1.0);
        let flags = reader.read_u64::<LittleEndian>()?;

        Ok(Self::new(right, up, at, position, flags))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self::new(
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Vector3> for Vector4 {
    fn from(vector: Vector3) -> Self {
        Self::new(vector.x, vector.y, vector.z, 0.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub(crate) fn decode_i32(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self::new(
            reader.read_i32::<LittleEndian>()? as u8,
            reader.read_i32::<LittleEndian>()? as u8,
            reader.read_i32::<LittleEndian>()? as u8,
        ))
    }

    pub(crate) fn decode_u8(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self::new(
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub(crate) fn decode_i32(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self::new(
            reader.read_i32::<LittleEndian>()? as u8,
            reader.read_i32::<LittleEndian>()? as u8,
            reader.read_i32::<LittleEndian>()? as u8,
            reader.read_i32::<LittleEndian>()? as u8,
        ))
    }

    pub(crate) fn decode_u8(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self::new(
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ))
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, 0)
    }
}

pub struct BoundingBox {
    pub supremum: Vector3,
    pub infimum: Vector3,
}

impl BoundingBox {
    pub fn new(supremum: Vector3, infimum: Vector3) -> Self {
        Self { supremum, infimum }
    }

    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self::new(
            Vector3::decode(reader)?,
            Vector3::decode(reader)?,
        ))
    }
}
