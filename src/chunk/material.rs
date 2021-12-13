use std::{
    io::{self, Read},
    mem, slice,
};

use crate::{hash, Matrix, Rgba};
use byteorder::{LittleEndian, ReadBytesExt};

pub struct Material {
    pub material_hash: u32,
    pub attributes: Attributes,
    pub textures: [MaterialTexture; 5],
}

impl Material {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Material> {
        let flags = reader.read_u32::<LittleEndian>()?;
        let _name_hash = reader.read_u32::<LittleEndian>()?;
        let additive_lighting_model = reader.read_i32::<LittleEndian>()? != 0;
        let colour = Rgba::decode_i32(reader)?;
        let specular = Rgba::decode_i32(reader)?;
        let power = reader.read_f32::<LittleEndian>()?;
        let shading_mode = reader.read_i32::<LittleEndian>()?;
        let blend = reader.read_i32::<LittleEndian>()? != 0;
        let blend_modes = BlendModes::decode(reader)?;
        let alpha_test = reader.read_i32::<LittleEndian>()? != 0;
        let alpha_test_mode = AlphaTestMode::decode(reader)?;
        let depth_buffer_write = reader.read_i32::<LittleEndian>()? != 0;
        let depth_buffer_comparison_mode = reader.read_i32::<LittleEndian>()?;
        let material_hash = reader.read_u32::<LittleEndian>()?;
        let owner = reader.read_u32::<LittleEndian>()?;
        let colour_buffer_write = reader.read_u32::<LittleEndian>()?;

        /*println!("Flags: {}", flags);
        println!("Additive lighting mode: {}", additive_lighting_model);
        println!("Colour: {:?}", colour);
        println!("Specular: {:?}", specular);
        println!("Power: {}", power);
        println!("Shading mode: {}", shading_mode);
        println!("Blend: {}", blend);
        println!("Blend modes: {:?}", blend_modes);
        println!("Alpha test: {}", alpha_test);
        println!("Alpha test mode: {:?}", alpha_test_mode);
        println!("Depth buffer write: {}", depth_buffer_write);
        println!("Depth buffer comparison mode: {}", depth_buffer_comparison_mode);
        println!("Material hash: {:X}", material_hash);
        println!("Owner: {}", owner);
        println!("Colour buffer write: {}", colour_buffer_write);*/

        let mut use_matrices = [false; 5];
        let mut matrices: [Option<Matrix>; 5] = Default::default();
        let mut generators = [0i32; 5];
        let mut uv_sets = [0u32; 5];
        let mut texture_hashes = [0u32; 5];
        let mut textures: [MaterialTexture; 5] = Default::default();

        for i in 0..5 {
            let uv_set = reader.read_u32::<LittleEndian>()?;
            //println!("UV set: {}", uv_set);
            let name_length = reader.read_i32::<LittleEndian>()?;
            //println!("Name length: {}", name_length);
            if name_length <= 0 {
                continue;
            }
            let mut name = Vec::with_capacity((name_length - 1) as usize);
            for _ in 0..name_length - 1 {
                name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
            }
            // Null terminator
            reader.read_i32::<LittleEndian>()?;
            let name = name.iter().collect::<String>();
            let format = reader.read_i32::<LittleEndian>()?;
            let filter = reader.read_i32::<LittleEndian>()?;
            let address = reader.read_i32::<LittleEndian>()?;
            let mask_name_length = reader.read_i32::<LittleEndian>()?;
            let mask_name = if mask_name_length > 0 {
                let mut mask_name = Vec::with_capacity((mask_name_length - 1) as usize);

                for _ in 0..mask_name_length - 1 {
                    mask_name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
                }

                // Null terminator
                reader.read_i32::<LittleEndian>()?;

                mask_name.into_iter().collect::<String>()
            } else {
                String::new()
            };
            let border_colour = Rgba::decode_i32(reader)?;
            let hash = reader.read_u32::<LittleEndian>()?;

            let texture = MaterialTexture {
                uv_set,
                name,
                format,
                address,
                mask_name,
                border_colour,
                hash,
            };

            uv_sets[i] = uv_set;
            texture_hashes[i] = hash;
            textures[i] = texture;
        }

        for i in 0..5 {
            let use_matrix = reader.read_i32::<LittleEndian>()? != 0;

            if use_matrix {
                let matrix = Matrix::decode(reader)?;

                matrices[i] = Some(matrix);
            }

            use_matrices[i] = use_matrix;
        }

        for i in 0..5 {
            let generator = reader.read_i32::<LittleEndian>()?;

            //println!("Generator: {}", generator);

            generators[i] = generator;
        }

        let envmap_type = reader.read_i32::<LittleEndian>()?;

        //println!("Envmap type: {}", envmap_type);

        let planar_sheer_envmap_distance = reader.read_f32::<LittleEndian>()?;

        //println!("Planar sheer envmap distance: {}", planar_sheer_envmap_distance);

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
            use_matrices,
            generators,
            uv_sets,
            texture_hashes,
            envmap_type,
            planar_sheer_envmap_distance,
        };

        Ok(Material {
            material_hash,
            attributes,
            textures,
        })
    }

    pub fn get_hash(&self) -> u32 {
        hash::hash(unsafe {
            slice::from_raw_parts(mem::transmute(&self), mem::size_of::<Attributes>())
        })
    }
}

#[repr(C)]
#[derive(Default)]
pub struct Attributes {
    pub flags: u32,
    pub additive_lighting_model: bool,
    pub colour: Rgba,
    pub specular: Rgba,
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
    pub use_matrices: [bool; 5],
    pub generators: [i32; 5],
    pub uv_sets: [u32; 5],
    pub texture_hashes: [u32; 5],
    pub envmap_type: i32,
    pub planar_sheer_envmap_distance: f32,
}

#[derive(Default)]
pub struct MaterialTexture {
    pub uv_set: u32,
    pub name: String,
    pub format: i32,
    pub address: i32,
    pub mask_name: String,
    pub border_colour: Rgba,
    pub hash: u32,
}

#[derive(Default, Debug)]
pub struct BlendModes {
    pub source_mode: i32,
    pub destionation_mode: i32,
}

impl BlendModes {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<BlendModes> {
        Ok(BlendModes {
            source_mode: reader.read_i32::<LittleEndian>()?,
            destionation_mode: reader.read_i32::<LittleEndian>()?,
        })
    }
}

#[derive(Default, Debug)]
pub struct AlphaTestMode {
    pub comparision_function: i32,
    pub reference: f32,
}

impl AlphaTestMode {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<AlphaTestMode> {
        Ok(AlphaTestMode {
            comparision_function: reader.read_i32::<LittleEndian>()?,
            reference: reader.read_f32::<LittleEndian>()?,
        })
    }
}
