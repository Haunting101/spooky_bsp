use crate::Decode;
use derive_new::new;
use std::io::Read;

#[derive(new, Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Decode for Vector3 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
        ))
    }
}

#[derive(new, Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Decode for Vector4 {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
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

#[derive(new)]
pub struct QuantizedQuaternion<T: Decode<Output = T>> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Decode<Output = T>> Decode for QuantizedQuaternion<T> {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            T::decode(reader, ())?,
            T::decode(reader, ())?,
            T::decode(reader, ())?,
            T::decode(reader, ())?,
        ))
    }
}

#[derive(new, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Matrix {
    pub right: Vector4,
    pub up: Vector4,
    pub at: Vector4,
    pub position: Vector4,
    pub flags: u64,
}

impl Decode for Matrix {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let right = Vector3::decode(reader, ())?.into();
        let up = Vector3::decode(reader, ())?.into();
        let at = Vector3::decode(reader, ())?.into();
        let position = (Vector3::decode(reader, ())?, 1.0).into();
        let flags = u64::decode(reader, ())?;

        Ok(Self::new(right, up, at, position, flags))
    }
}

#[derive(new, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Decode for Plane {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
            f32::decode(reader, ())?,
        ))
    }
}

#[derive(new, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QuantizedPlane {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub flags: u8,
    pub d: f32,
}

impl Decode for QuantizedPlane {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
            u8::decode(reader, ())?,
            f32::decode(reader, ())?,
        ))
    }
}

#[derive(new, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Decode for Rectangle {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(Self::new(
            i32::decode(reader, ())?,
            i32::decode(reader, ())?,
            i32::decode(reader, ())?,
            i32::decode(reader, ())?,
        ))
    }
}
