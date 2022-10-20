use crate::{Decode, Rectangle, Rgba};

use derive_new::new;
use std::io::Read;

const STORE_LAYER_REMAP_COUNT: u32 = 3;
const UPDATE_SUBRECTS_COUNT: u32 = 2;

#[derive(Clone, Debug)]
pub struct SwitchableLights {}

impl Decode for SwitchableLights {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let magic = u32::decode(reader, ())?;

        assert!(magic <= 3);

        let gamma_ramp_power = if magic >= 1 {
            f32::decode(reader, ())?
        } else {
            4.0
        };

        if magic >= STORE_LAYER_REMAP_COUNT {
            let source_layer_count = u32::decode(reader, ())?;
            let layer_remap_table = (0..source_layer_count)
                .into_iter()
                .map(|_| u32::decode(reader, ()))
                .collect::<Result<Vec<_>, _>>()?;
        }

        let light_map_count = u32::decode(reader, ())?;
        let light_maps = (0..light_map_count)
            .into_iter()
            .map(|_| SwitchableLightMap::decode(reader, magic))
            .collect::<Result<Vec<_>, _>>()?;
        let light_data_count = u32::decode(reader, ())?;
        let light_data = (0..light_data_count)
            .into_iter()
            .map(|_| SwitchableLightData::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;
        let material_block_count = u32::decode(reader, ())?;
        let material_blocks = (0..material_block_count)
            .into_iter()
            .map(|_| MaterialBlockSwitchInfo::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {})
    }
}

#[derive(Clone, Debug)]
pub struct SwitchableLightMap {}

impl SwitchableLightMap {}

impl Decode<u32> for SwitchableLightMap {
    fn decode(reader: &mut impl Read, magic: u32) -> eyre::Result<Self> {
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

        Ok(Self {})
    }
}

#[derive(Clone, Debug)]
pub struct LightMapUpdateBlock {}

impl Decode<i32> for LightMapUpdateBlock {
    fn decode(reader: &mut impl Read, pixels_to_read: i32) -> eyre::Result<Self> {
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

        Ok(Self {})
    }
}

#[derive(new, Clone, Debug)]
pub struct SwitchableLightData {
    pub dependent_light_maps: Vec<u32>,
    pub vertex_blocks: Vec<SingleVertexSwitchBlock>,
}

impl Decode for SwitchableLightData {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let dependent_light_map_count = u32::decode(reader, ())?;
        let dependent_light_maps = (0..dependent_light_map_count)
            .into_iter()
            .map(|_| u32::decode(reader, ())) // Indices
            .collect::<Result<Vec<_>, _>>()?;

        let vertex_block_count = u32::decode(reader, ())?;
        let vertex_blocks = (0..vertex_block_count)
            .into_iter()
            .map(|_| SingleVertexSwitchBlock::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SwitchableLightData::new(
            dependent_light_maps,
            vertex_blocks,
        ))
    }
}

#[derive(new, Clone, Debug)]
pub struct SingleVertexSwitchBlock {
    pub material_block_index: u32,
    pub updates: Vec<UpdateRGBA>,
}

impl Decode for SingleVertexSwitchBlock {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let material_block_index = u32::decode(reader, ())?;
        let update_vertices_count = u32::decode(reader, ())?;
        let updates = (0..update_vertices_count)
            .into_iter()
            .map(|_| UpdateRGBA::decode(reader, ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SingleVertexSwitchBlock::new(material_block_index, updates))
    }
}

#[derive(new, Clone, Debug)]
pub struct UpdateRGBA {
    pub vertex_index: u32,
    pub color: Rgba,
}

impl Decode for UpdateRGBA {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let vertex_index = u32::decode(reader, ())?;
        let color = Rgba::decode(reader, ())?;

        Ok(UpdateRGBA::new(vertex_index, color))
    }
}

#[derive(new, Clone, Debug)]
pub struct MaterialBlockSwitchInfo {
    pub lighting_id: u32,
    pub is_world_geometry: bool,
}

impl Decode for MaterialBlockSwitchInfo {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let lighting_id = u32::decode(reader, ())?;
        let is_world_geometry = u32::decode(reader, ())? == 1;
        let vertices_count = u32::decode(reader, ())?;
        let current_vertex_light = vec![NonSatRGB::default(); vertices_count as usize];

        Ok(MaterialBlockSwitchInfo::new(lighting_id, is_world_geometry))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NonSatRGB {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}
