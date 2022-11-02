use std::io::Read;

use crate::{ChunkHeader, Decode, DecodeError, Rgba};

#[derive(Clone, Debug)]
pub struct Light {
    pub base_flags: u32,
    pub light_type: i32,
    pub flags: u32,
    pub radius: f32,
    pub light_color: Rgba,
    pub cone_angle: f32,
    pub photon_light_abs_scale: f32,
    pub light_switch_layer_index: Option<u32>,
}

impl Decode<&ChunkHeader> for Light {
    fn decode(reader: &mut impl Read, chunk_header: &ChunkHeader) -> Result<Self, DecodeError> {
        let base_flags = u32::decode(reader, ())?;
        let light_type = i32::decode(reader, ())?;
        let flags = u32::decode(reader, ())?;
        let radius = f32::decode(reader, ())?;
        let light_color = Rgba::decode(reader, ())?;
        let cone_angle = f32::decode(reader, ())?;
        let photon_light_abs_scale = f32::decode(reader, ())?;
        let light_switch_layer_index = if chunk_header.get_version() >= 0x666 + 0x39 {
            Some(u32::decode(reader, ())?)
        } else {
            None
        };

        Ok(Self {
            base_flags,
            light_type,
            flags,
            radius,
            light_color,
            cone_angle,
            photon_light_abs_scale,
            light_switch_layer_index,
        })
    }
}
