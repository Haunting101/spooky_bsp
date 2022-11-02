use crate::{Decode, DecodeError, Rectangle, Rgba};

use std::io::Read;

const STORE_LAYER_REMAP_COUNT: u32 = 3;
const UPDATE_SUBRECTS_COUNT: u32 = 2;

#[derive(Clone, Debug)]
pub struct SwitchableLights {
    pub gamma_ramp_power: f32,
    pub layer_remap_table: Option<Vec<u32>>,
    pub light_maps: Vec<SwitchableLightMap>,
    pub light_data: Vec<SwitchableLightData>,
    pub material_blocks: Vec<MaterialBlockSwitchInfo>,
}

impl Decode for SwitchableLights {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let magic = u32::decode(reader, ())?;

        assert!(magic <= 3);

        let gamma_ramp_power = if magic >= 1 {
            f32::decode(reader, ())?
        } else {
            4.0
        };

        let layer_remap_table = if magic >= STORE_LAYER_REMAP_COUNT {
            Some(Vec::decode(reader, ())?)
        } else {
            None
        };

        let light_maps = Vec::decode(reader, magic)?;
        let light_data = Vec::decode(reader, ())?;
        let material_blocks = Vec::decode(reader, ())?;

        Ok(Self {
            gamma_ramp_power,
            layer_remap_table,
            light_maps,
            light_data,
            material_blocks,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SwitchableLightMap {
    pub texture_hash: u32,
    pub name: String,
    pub update_region: Rectangle,
    pub update_blocks: Vec<LightMapUpdateBlock>,
}

impl Decode<u32> for SwitchableLightMap {
    fn decode(reader: &mut impl Read, magic: u32) -> Result<Self, DecodeError> {
        let texture_hash = u32::decode(reader, ())?;
        let name = (0..12)
            .into_iter()
            .map(|_| char::decode(reader, ()))
            .collect::<Result<String, _>>()?;
        let update_region = Rectangle::decode(reader, ())?;
        let update_block_count = u32::decode(reader, ())?;
        let pixels_to_read = if magic >= UPDATE_SUBRECTS_COUNT {
            0
        } else {
            update_region.width * update_region.height
        };

        let update_blocks = (0..update_block_count)
            .into_iter()
            .map(|_| LightMapUpdateBlock::decode(reader, pixels_to_read))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            texture_hash,
            name,
            update_region,
            update_blocks,
        })
    }
}

#[derive(Clone, Debug)]
pub struct LightMapUpdateBlock {
    pub layer_index: u32,
    pub additive_data: Vec<Rgba>,
}

impl Decode<i32> for LightMapUpdateBlock {
    fn decode(reader: &mut impl Read, pixels_to_read: i32) -> Result<Self, DecodeError> {
        let layer_index = u32::decode(reader, ())?;

        let pixels = if pixels_to_read == 0 {
            let update_sub_rectangle = Rectangle::decode(reader, ())?;

            update_sub_rectangle.width * update_sub_rectangle.height
        } else {
            pixels_to_read
        };

        let additive_data = (0..pixels)
            .into_iter()
            .map(|_| Rgba::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            layer_index,
            additive_data,
        })
    }
}

#[derive(Clone, Debug, Decode)]
pub struct SwitchableLightData {
    pub dependent_light_maps: Vec<u32>,
    pub vertex_blocks: Vec<SingleVertexSwitchBlock>,
}

#[derive(Clone, Debug, Decode)]
pub struct SingleVertexSwitchBlock {
    pub material_block_index: u32,
    pub updates: Vec<UpdateRGBA>,
}

#[derive(Clone, Debug, Decode)]
pub struct UpdateRGBA {
    pub vertex_index: u32,
    pub color: Rgba,
}

#[derive(Clone, Debug, Decode)]
pub struct MaterialBlockSwitchInfo {
    pub lighting_id: u32,
    pub is_world_geometry: bool,
    pub vertices_count: u32,
}
