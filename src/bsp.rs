use crate::{Material, Mesh, ModelPart, Occlusion, SectorOctree, Texture, World};

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
