use derive_new::new;
use std::io::Read;

use crate::{Decode, I32Encoded, Matrix, NullTerminated, Rgba};

#[derive(new, Clone, Debug)]
pub struct Material {
    pub material_hash: u32,
    pub attributes: Attributes,
    pub textures: [MaterialTexture; 5],
}

impl Decode for Material {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let flags = u32::decode(reader, ())?;
        let _name_hash = u32::decode(reader, ())?;
        let additive_lighting_model = bool::decode(reader, ())?;
        let colour = Rgba::<i32>::decode(reader, ())?;
        let specular = Rgba::<i32>::decode(reader, ())?;
        let power = f32::decode(reader, ())?;
        let shading_mode = i32::decode(reader, ())?;
        let blend = bool::decode(reader, ())?;
        let blend_modes = BlendModes::decode(reader, ())?;
        let alpha_test = bool::decode(reader, ())?;
        let alpha_test_mode = AlphaTestMode::decode(reader, ())?;
        let depth_buffer_write = bool::decode(reader, ())?;
        let depth_buffer_comparison_mode = i32::decode(reader, ())?;
        let material_hash = u32::decode(reader, ())?;
        let owner = u32::decode(reader, ())?;
        let colour_buffer_write = u32::decode(reader, ())?;

        let textures = <[MaterialTexture; 5]>::decode(reader, ())?;
        let matrices = <[Option<Matrix>; 5]>::decode(reader, ())?;
        let generators = <[i32; 5]>::decode(reader, ())?;

        let envmap_type = i32::decode(reader, ())?;
        let planar_sheer_envmap_distance = f32::decode(reader, ())?;

        let attributes = Attributes {
            flags,
            additive_lighting_model,
            colour,
            specular,
            power,
            shading_mode,
            blend,
            blend_modes,
            alpha_test,
            alpha_test_mode,
            depth_buffer_write,
            depth_buffer_comparison_mode,
            owner,
            colour_buffer_write,
            generators,
            envmap_type,
            planar_sheer_envmap_distance,
        };

        Ok(Material::new(material_hash, attributes, textures))
    }
}

#[derive(new, Clone, Debug, Default)]
pub struct Attributes {
    pub flags: u32,
    pub additive_lighting_model: bool,
    pub colour: Rgba<i32>,
    pub specular: Rgba<i32>,
    pub power: f32,
    pub shading_mode: i32,
    pub depth_buffer_write: bool,
    pub depth_buffer_comparison_mode: i32,
    pub blend: bool,
    pub blend_modes: BlendModes,
    pub alpha_test: bool,
    pub alpha_test_mode: AlphaTestMode,
    pub owner: u32,
    pub colour_buffer_write: u32,
    pub generators: [i32; 5],
    pub envmap_type: i32,
    pub planar_sheer_envmap_distance: f32,
}

#[derive(new, Clone, Debug, Default)]
pub struct MaterialTexture {
    pub uv_set: u32,
    pub name: String,
    pub format: i32,
    pub filter: i32,
    pub address: i32,
    pub mask_name: String,
    pub border_colour: Rgba<i32>,
    pub hash: u32,
}

impl Decode for MaterialTexture {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let uv_set = u32::decode(reader, ())?;
        let name = NullTerminated::<I32Encoded<String>>::decode(reader, ())?;

        if name.len() > 0 {
            let format = i32::decode(reader, ())?;
            let filter = i32::decode(reader, ())?;
            let address = i32::decode(reader, ())?;
            let mask_name = NullTerminated::<I32Encoded<String>>::decode(reader, ())?;
            let border_colour = Rgba::<i32>::decode(reader, ())?;
            let hash = u32::decode(reader, ())?;

            Ok(MaterialTexture::new(
                uv_set,
                name,
                format,
                filter,
                address,
                mask_name,
                border_colour,
                hash,
            ))
        } else {
            let format = 0;
            let filter = 0;
            let address = 0;
            let mask_name = String::new();
            let border_colour = Rgba::<i32>::default();
            let hash = 0;

            Ok(MaterialTexture::new(
                uv_set,
                name,
                format,
                filter,
                address,
                mask_name,
                border_colour,
                hash,
            ))
        }
    }
}

#[derive(new, Clone, Debug, Default)]
pub struct BlendModes {
    pub source_mode: i32,
    pub destionation_mode: i32,
}

impl Decode for BlendModes {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(BlendModes::new(
            i32::decode(reader, ())?,
            i32::decode(reader, ())?,
        ))
    }
}

#[derive(new, Clone, Debug, Default)]
pub struct AlphaTestMode {
    pub comparision_function: i32,
    pub reference: f32,
}

impl Decode for AlphaTestMode {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        Ok(AlphaTestMode::new(
            i32::decode(reader, ())?,
            f32::decode(reader, ())?,
        ))
    }
}
