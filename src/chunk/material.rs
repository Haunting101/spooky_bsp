use std::io::Read;

use crate::{Decode, DecodeError, I32Encoded, Matrix, NullTerminated, Rgba};

#[derive(Clone, Debug, Decode)]
pub struct Material {
    pub flags: u32,
    pub name_hash: u32,
    pub additive_lighting_model: bool,
    #[encoding(i32)]
    pub color: Rgba,
    #[encoding(i32)]
    pub specular: Rgba,
    pub power: f32,
    pub shading_mode: i32,
    pub blend: bool,
    pub blend_modes: BlendModes,
    pub alpha_test: bool,
    pub alpha_test_mode: AlphaTestMode,
    pub depth_buffer_write: bool,
    pub depth_buffer_comparison_mode: i32,
    pub material_hash: u32,
    pub owner: u32,
    pub color_buffer_write: u32,
    pub textures: [MaterialTexture; 5],
    pub matrices: [Option<Matrix>; 5],
    pub generators: [i32; 5],
    pub envmap_type: i32,
    pub planar_sheer_envmap_distance: f32,
}

#[derive(Clone, Debug, Default)]
pub struct MaterialTexture {
    pub uv_set: u32,
    pub name: String,
    pub format: Option<i32>,
    pub filter: Option<i32>,
    pub address: Option<i32>,
    pub mask_name: Option<String>,
    pub border_color: Option<Rgba>,
    pub hash: Option<u32>,
}

impl Decode for MaterialTexture {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let uv_set = u32::decode(reader, ())?;
        let name = I32Encoded::<NullTerminated<String>>::decode(reader, ())?;

        let (format, filter, address, mask_name, border_color, hash) = if name.len() > 0 {
            let format = i32::decode(reader, ())?;
            let filter = i32::decode(reader, ())?;
            let address = i32::decode(reader, ())?;
            let mask_name = I32Encoded::<NullTerminated<String>>::decode(reader, ())?;
            let border_color = I32Encoded::<Rgba>::decode(reader, ())?;
            let hash = u32::decode(reader, ())?;

            (
                Some(format),
                Some(filter),
                Some(address),
                Some(mask_name),
                Some(border_color),
                Some(hash),
            )
        } else {
            (None, None, None, None, None, None)
        };

        Ok(Self {
            uv_set,
            name,
            format,
            filter,
            address,
            mask_name,
            border_color,
            hash,
        })
    }
}

#[derive(Clone, Debug, Decode, Default)]
pub struct BlendModes {
    pub source_mode: i32,
    pub destination_mode: i32,
}

#[derive(Clone, Debug, Decode, Default)]
pub struct AlphaTestMode {
    pub comparision_function: i32,
    pub reference: f32,
}
