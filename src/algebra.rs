use crate::Decode;
use std::io::Read;

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
}

impl Decode for Vector3 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(Self::new(
            f32::decode(reader)?,
            f32::decode(reader)?,
            f32::decode(reader)?,
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

impl Decode for Vector4 {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(Self::new(
            f32::decode(reader)?,
            f32::decode(reader)?,
            f32::decode(reader)?,
            f32::decode(reader)?,
        ))
    }
}

impl From<Vector3> for Vector4 {
    fn from(vector: Vector3) -> Self {
        Self::new(vector.x, vector.y, vector.z, 0.0)
    }
}

impl From<(Vector3, f32)> for Vector4 {
    fn from((vector, w): (Vector3, f32)) -> Self {
        Self::new(vector.x, vector.y, vector.z, w)
    }
}

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
}

impl Decode for Matrix {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let right = Vector3::decode(reader)?.into();
        let up = Vector3::decode(reader)?.into();
        let at = Vector3::decode(reader)?.into();
        let position = (Vector3::decode(reader)?, 1.0).into();
        let flags = u64::decode(reader)?;

        Ok(Self::new(right, up, at, position, flags))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }
}

impl Decode for Plane {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(Self::new(
            f32::decode(reader)?,
            f32::decode(reader)?,
            f32::decode(reader)?,
            f32::decode(reader)?,
        ))
    }
}
