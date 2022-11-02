use crate::{Decode, DecodeError};
use std::io::Read;

#[derive(Clone, Copy, Debug, Decode, Default, PartialEq, PartialOrd)]
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

#[derive(Clone, Copy, Debug, Decode, Default, PartialEq, PartialOrd)]
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
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: 0.0,
        }
    }
}

impl From<(Vector3, f32)> for Vector4 {
    fn from((vector, w): (Vector3, f32)) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w,
        }
    }
}

#[derive(Clone, Debug)]
pub struct QuantizedQuaternion<T: Decode<Output = T>> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Decode<Output = T>> QuantizedQuaternion<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Decode<Output = T>> Decode for QuantizedQuaternion<T> {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(Self {
            x: T::decode(reader, ())?,
            y: T::decode(reader, ())?,
            z: T::decode(reader, ())?,
            w: T::decode(reader, ())?,
        })
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

impl Decode for Matrix {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let right = Vector3::decode(reader, ())?.into();
        let up = Vector3::decode(reader, ())?.into();
        let at = Vector3::decode(reader, ())?.into();
        let position = (Vector3::decode(reader, ())?, 1.0).into();
        let flags = u64::decode(reader, ())?;

        Ok(Self {
            right,
            up,
            at,
            position,
            flags,
        })
    }
}

#[derive(Clone, Debug, Decode, Default, PartialEq, PartialOrd)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

#[derive(Clone, Debug, Decode, Default, PartialEq, PartialOrd)]
pub struct QuantizedPlane {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub flags: u8,
    pub d: f32,
}

#[derive(Clone, Debug, Decode, Default, PartialEq, PartialOrd)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
