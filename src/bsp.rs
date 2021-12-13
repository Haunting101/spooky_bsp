use crate::{
    ChunkHeader, ChunkType, Decode, Material, Mesh, ModelPart, Occlusion, SectorOctree, Texture,
    World,
};
use flate2::read::GzDecoder;
use std::io::Read;

pub struct Bsp {
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub worlds: Vec<World>,
    pub meshes: Vec<Mesh>,
    pub model_parts: Vec<ModelPart>,
    pub octree_sectors: Vec<SectorOctree>,
    pub occlusions: Vec<Occlusion>,
}

impl Bsp {
    pub fn new(
        textures: Vec<Texture>,
        materials: Vec<Material>,
        worlds: Vec<World>,
        meshes: Vec<Mesh>,
        model_parts: Vec<ModelPart>,
        octree_sectors: Vec<SectorOctree>,
        occlusions: Vec<Occlusion>,
    ) -> Self {
        Self {
            textures,
            materials,
            worlds,
            meshes,
            model_parts,
            octree_sectors,
            occlusions,
        }
    }
}

impl Decode for Bsp {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        let mut decoder = GzDecoder::new(reader);

        let mut textures = Vec::new();
        let mut materials = Vec::new();
        let mut worlds = Vec::new();
        let mut meshes = Vec::new();
        let mut model_parts = Vec::new();
        let mut octree_sectors = Vec::new();
        let mut occlusions = Vec::new();

        let mut material_count = 0;

        while let Ok(chunk_header) = ChunkHeader::decode(&mut decoder) {
            match chunk_header.get_chunk_type() {
                ChunkType::Textures => textures = Vec::decode(&mut decoder)?,
                ChunkType::Materials => material_count = i32::decode(&mut decoder)?,
                ChunkType::MaterialObj => materials.push(Material::decode(&mut decoder)?),
                ChunkType::World => worlds.push(World::decode(&mut decoder)?),
                ChunkType::ModelGroup => meshes.push(Mesh::decode(&mut decoder)?),
                ChunkType::SPMesh => model_parts.push(ModelPart::decode(&mut decoder)?),
                ChunkType::SectorOctree => octree_sectors.push(SectorOctree::decode(&mut decoder)?),
                ChunkType::Occlusion => occlusions.push(Occlusion::decode(&mut decoder)?),
                _ => decoder.read_exact(vec![0u8; chunk_header.get_size() as usize].as_mut())?,
            }
        }

        Ok(Bsp::new(
            textures,
            materials,
            worlds,
            meshes,
            model_parts,
            octree_sectors,
            occlusions,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn decode_file_test() {
        Bsp::decode(&mut File::open("Darkling.bsp").unwrap()).unwrap();
    }
}
