use crate::{BoundingBox, Decode, DecodeError, QuantizedQuaternion, Vector3};

use num_enum::TryFromPrimitive;
use std::io::Read;

#[derive(Clone, Debug)]
pub struct AnimationKey {
    pub type_: i32,
    pub target_hash: u32,
    pub time_step: f32,
    pub material_block_index: u16,
    pub bounding_box_maximum: Option<BoundingBox>,
    pub interpolation_type: Interpolation,
    pub times: Option<Vec<f32>>,
    pub keys: AnimationKeys,
    pub adaptive_differential_pulse_code_modulation:
        Option<AdaptiveDifferentialPulseCodeModulation>,
}

impl Decode for AnimationKey {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let type_ = i32::decode(reader, ())?;
        let target_hash = u32::decode(reader, ())?;
        let time_step = f32::decode(reader, ())?;
        let key_count = i32::decode(reader, ())?;
        let material_block_index = u16::decode(reader, ())?;
        let bounding_box_maximum = Option::<BoundingBox>::decode(reader, ())?;

        let interpolation_type = Interpolation::decode(reader, ())?;

        let times = {
            let has_times = bool::decode(reader, ())?;

            if has_times {
                let times = (0..key_count)
                    .map(|_| f32::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                Some(times)
            } else {
                None
            }
        };

        let keys = match AnimationKeyType::try_from(type_)? {
            AnimationKeyType::Rotate => {
                let rotations = (0..key_count)
                    .map(|_| QuantizedQuaternion::<i32>::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                AnimationKeys::Rotations(rotations)
            }
            AnimationKeyType::Translate => {
                let translations = (0..key_count)
                    .map(|_| Vector3::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                AnimationKeys::Translations(translations)
            }
            AnimationKeyType::Shape => {
                let shapes = (0..key_count)
                    .map(|_| Shape::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                AnimationKeys::Shapes(shapes)
            }
            AnimationKeyType::Uv => {
                let uvs = (0..key_count)
                    .into_iter()
                    .map(|_| {
                        (0..2)
                            .into_iter()
                            .map(|_| {
                                let uv_count = u16::decode(reader, ()).unwrap();

                                let us = (0..uv_count)
                                    .into_iter()
                                    .map(|_| u16::decode(reader, ()))
                                    .collect::<Result<Vec<_>, _>>()
                                    .unwrap();

                                let vs = (0..uv_count)
                                    .into_iter()
                                    .map(|_| u16::decode(reader, ()))
                                    .collect::<Result<Vec<_>, _>>()
                                    .unwrap();

                                us.into_iter()
                                    .zip(vs.into_iter())
                                    .map(|(u, v)| Uv::new(u, v))
                                    .collect::<Vec<_>>()
                            })
                            .flatten()
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<Vec<_>>();

                AnimationKeys::Uvs(uvs)
            }
            AnimationKeyType::VisibilityState => {
                let visibility_states = (0..key_count)
                    .map(|_| VisibilityState::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                AnimationKeys::VisibilityStates(visibility_states)
            }
        };

        let adaptive_differential_pulse_code_modulation =
            Option::<AdaptiveDifferentialPulseCodeModulation>::decode(reader, ())?;

        Ok(Self {
            type_,
            target_hash,
            time_step,
            material_block_index,
            bounding_box_maximum,
            interpolation_type,
            times,
            keys,
            adaptive_differential_pulse_code_modulation,
        })
    }
}

#[derive(Clone, Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(i32)]
pub enum Interpolation {
    Linear = 0,
    CubicSpline = 1,
}

impl Decode for Interpolation {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(Self::try_from(i32::decode(reader, ())?)?)
    }
}

#[derive(Clone, Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(i32)]
pub enum AnimationKeyType {
    Rotate = 0,
    Translate = 1,
    Shape = 2,
    Uv = 3,
    VisibilityState = 4,
}

#[derive(Clone, Debug)]
pub enum AnimationKeys {
    Rotations(Vec<QuantizedQuaternion<i32>>),
    Translations(Vec<Vector3>),
    Shapes(Vec<Shape>),
    Uvs(Vec<Uv>),
    VisibilityStates(Vec<VisibilityState>),
}

#[derive(Clone, Debug)]
pub enum Shape {
    KeyFrame {
        animated_vertices: KeyFrameAnimatedVertices,
        normals: KeyFrameNormals,
    },
    NotKeyFrame {
        animated_vertices: NotKeyFrameAnimatedVertices,
        normals: NotKeyFrameNormals,
    },
}

impl Decode for Shape {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let key_frame = bool::decode(reader, ())?;

        if key_frame {
            let animated_vertices = {
                let animated_vertices_count = u16::decode(reader, ())?;

                let elements = (0..animated_vertices_count)
                    .into_iter()
                    .map(|_| Vector3::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                KeyFrameAnimatedVertices { elements }
            };

            let normals = {
                let normals_count = u16::decode(reader, ())?;

                let elements = (0..normals_count)
                    .into_iter()
                    .map(|_| Vector3::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                KeyFrameNormals { elements }
            };

            Ok(Self::KeyFrame {
                animated_vertices,
                normals,
            })
        } else {
            let animated_vertices = {
                let animated_vertices_count = u16::decode(reader, ())?;

                let indices = (0..animated_vertices_count)
                    .into_iter()
                    .map(|_| u16::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;
                let elements = (0..animated_vertices_count)
                    .into_iter()
                    .map(|_| u16::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                NotKeyFrameAnimatedVertices { indices, elements }
            };

            let normals = {
                let normals_count = u16::decode(reader, ())?;

                let indices = (0..normals_count)
                    .into_iter()
                    .map(|_| u16::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;
                let elements = (0..normals_count)
                    .into_iter()
                    .map(|_| u16::decode(reader, ()))
                    .collect::<Result<Vec<_>, _>>()?;

                NotKeyFrameNormals { indices, elements }
            };

            Ok(Self::NotKeyFrame {
                animated_vertices,
                normals,
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeyFrameAnimatedVertices {
    pub elements: Vec<Vector3>,
}

#[derive(Clone, Debug)]
pub struct NotKeyFrameAnimatedVertices {
    pub indices: Vec<u16>,
    pub elements: Vec<u16>,
}

#[derive(Clone, Debug)]
pub struct KeyFrameNormals {
    pub elements: Vec<Vector3>,
}

#[derive(Clone, Debug)]
pub struct NotKeyFrameNormals {
    pub indices: Vec<u16>,
    pub elements: Vec<u16>,
}

#[derive(Clone, Debug)]
pub struct Uv {
    pub u: u16,
    pub v: u16,
}

impl Uv {
    pub fn new(u: u16, v: u16) -> Self {
        Self { u, v }
    }
}

#[derive(Clone, Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum VisibilityState {
    Off = 0,
    On = 1,
}

impl Decode for VisibilityState {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(Self::try_from(u8::decode(reader, ())?)?)
    }
}

#[derive(Clone, Debug, Decode)]
pub struct AdaptiveDifferentialPulseCodeModulation {
    pub vertex_type: AdaptiveDifferentialPulseCodeModulationType,
    pub normal_type: AdaptiveDifferentialPulseCodeModulationType,
    pub vertex_range: Vector3,
    pub normal_range: Vector3,
}

#[derive(Clone, Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(i32)]
pub enum AdaptiveDifferentialPulseCodeModulationType {
    None = 0,
    Linear = 1,
    Exponential = 2,
}

impl Decode for AdaptiveDifferentialPulseCodeModulationType {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        Ok(Self::try_from(i32::decode(reader, ())?)?)
    }
}
