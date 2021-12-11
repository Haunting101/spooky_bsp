use std::{error::Error, io::Read};

use crate::{ChunkHeader, ChunkType, Material, Texture, World, Mesh, ModelPart, SectorOctree, Occlusion, Bsp};
use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Bytes;
use flate2::read::GzDecoder;

pub struct BspDecoder {
    bytes: Bytes,
}

impl BspDecoder {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    pub fn decode(&self) -> Result<Bsp, Box<dyn Error>> {
        let mut decoder = GzDecoder::new(self.bytes.as_ref());

        let mut textures = Vec::new();
        let mut materials = Vec::new();
        let mut worlds = Vec::new();
        let mut meshes = Vec::new();
        let mut model_parts = Vec::new();
        let mut octree_sectors = Vec::new();
        let mut occlusions = Vec::new();

        while let Ok(chunk_header) = ChunkHeader::decode(&mut decoder) {
            //println!("{:?}", chunk_header);

            match chunk_header.get_chunk_type() {
                ChunkType::Textures => {
                    let textures_count = decoder.read_i32::<LittleEndian>()?;
                    
                    textures.reserve(textures_count as usize);

                    for _ in 0..textures_count {
                        let texture = Texture::decode(&mut decoder)?;

                        textures.push(texture);
                    }
                }
                ChunkType::Materials => {
                    let materials_count = decoder.read_i32::<LittleEndian>()?;
                }
                ChunkType::MaterialObj => {
                    let material = Material::decode(&mut decoder)?;

                    materials.push(material);
                },
                ChunkType::World => {
                    let world = World::decode(&mut decoder)?;

                    worlds.push(world);
                },
                ChunkType::ModelGroup => {
                    let mesh = Mesh::decode(&mut decoder)?;

                    meshes.push(mesh);
                },
                ChunkType::SPMesh => {
                    let model_part = ModelPart::decode(&mut decoder)?;

                    model_parts.push(model_part);
                },
                ChunkType::SectorOctree => {
                    let sector_octree = SectorOctree::decode(&mut decoder)?;

                    octree_sectors.push(sector_octree);
                },
                ChunkType::Occlusion => {
                    let occlusion = Occlusion::decode(&mut decoder)?;

                    occlusions.push(occlusion);
                },
                _ => decoder.read_exact(vec![0u8; chunk_header.get_size() as usize].as_mut())?,
            }
        }

        let bsp = Bsp::new(textures, materials, worlds, meshes, model_parts, octree_sectors, occlusions);

        Ok(bsp)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn decode_file_test() {
        let decoder = BspDecoder::new(Bytes::copy_from_slice(&fs::read("Darkling.bsp").unwrap()));

        decoder.decode().unwrap();
    }
}
