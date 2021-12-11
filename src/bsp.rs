use crate::{Material, Mesh, ModelPart, Occlusion, SectorOctree, Texture, World};

pub struct Bsp {
    textures: Vec<Texture>,
    materials: Vec<Material>,
    worlds: Vec<World>,
    meshes: Vec<Mesh>,
    model_parts: Vec<ModelPart>,
    octree_sectors: Vec<SectorOctree>,
    occlusions: Vec<Occlusion>,
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
