mod animation_dictionary;
mod animation_key;
mod atomic_mesh;
mod camera_projection;
mod clips;
mod clump;
mod collision;
mod entities;
mod entity;
mod frame;
mod frame_child;
mod light;
mod material;
mod mesh;
mod model_part;
mod navigation_mesh;
mod ngon_list;
mod null_box;
mod nulls;
mod occlusion;
mod sector_octree;
mod spline;
mod switchable_lights;
mod texture;
mod world;
mod zones;

pub use animation_dictionary::*;
pub use animation_key::*;
pub use atomic_mesh::*;
pub use camera_projection::*;
pub use clips::*;
pub use clump::*;
pub use collision::*;
pub use entities::*;
pub use entity::*;
pub use frame::*;
pub use frame_child::*;
pub use light::*;
pub use material::*;
pub use mesh::*;
pub use model_part::*;
pub use navigation_mesh::*;
pub use ngon_list::*;
pub use null_box::*;
pub use nulls::*;
pub use occlusion::*;
pub use sector_octree::*;
pub use spline::*;
pub use switchable_lights::*;
pub use texture::*;
pub use world::*;
pub use zones::*;

use crate::Decode;
use std::io::Read;

use num_enum::TryFromPrimitive;

pub enum Chunk {
    GLProject(CameraProjection),
    MaterialObj(Material),
    ModelGroup(Mesh),
    BoneObj(Frame),
    SPMesh(ModelPart),
    Collision(Collision),
    AtomicMesh(AtomicMesh),
    SkinObj(Clump),
    GLCamera(CameraProjection),
    LightObj(Light),
    LevelObj(FrameChild),
    Materials(i32),
    SectorOctree(SectorOctree),
    World(World),
    AnimationKey(AnimationKey),
    AnimLib(AnimationDictionary),
    OcclusionMesh(NGonList),
    Occlusion(Occlusion),
    WpPoints(Nulls),
    NavigationMesh(NavigationMesh),
    Zones(Zones),
    Area(Spline),
    LinkEmm(NullBox),
    Animation(Clips),
    SpLights(SwitchableLights),
    Entities(Entities),
    Entity(Entity),
    Textures(Vec<Texture>),
}

impl Decode<Option<&World>> for Chunk {
    fn decode(reader: &mut impl Read, world: Option<&World>) -> eyre::Result<Self> {
        let chunk_header = ChunkHeader::decode(reader, ())?;

        Ok(match chunk_header.get_chunk_type() {
            ChunkType::Textures => Chunk::Textures(Vec::decode(reader, ())?),
            ChunkType::Materials => {
                let material_count = i32::decode(reader, ())?;

                assert!(material_count >= 0);

                Chunk::Materials(material_count)
            }
            ChunkType::MaterialObj => Chunk::MaterialObj(Material::decode(reader, ())?),
            ChunkType::World => Chunk::World(World::decode(reader, ())?),
            ChunkType::ModelGroup => Chunk::ModelGroup(Mesh::decode(reader, ())?),
            ChunkType::SPMesh => Chunk::SPMesh(ModelPart::decode(reader, ())?),
            ChunkType::SectorOctree => Chunk::SectorOctree(SectorOctree::decode(reader, ())?),
            ChunkType::Occlusion => Chunk::Occlusion(Occlusion::decode(reader, ())?),
            ChunkType::LevelObj => Chunk::LevelObj(FrameChild::decode(reader, ())?),
            ChunkType::LinkEmm => Chunk::LinkEmm(NullBox::decode(reader, ())?),
            ChunkType::AtomicMesh => Chunk::AtomicMesh(AtomicMesh::decode(reader, ())?),
            ChunkType::GLCamera => Chunk::GLCamera(CameraProjection::decode(reader, ())?),
            ChunkType::GLProject => Chunk::GLProject(CameraProjection::decode(reader, ())?),
            ChunkType::LightObj => Chunk::LightObj(Light::decode(reader, &chunk_header)?),
            ChunkType::OcclusionMesh => Chunk::OcclusionMesh(NGonList::decode(reader, ())?),
            ChunkType::Area => Chunk::Area(Spline::decode(reader, ())?),
            ChunkType::BoneObj => Chunk::BoneObj(Frame::decode(reader, ())?),
            ChunkType::WpPoints => Chunk::WpPoints(Nulls::decode(reader, ())?),
            ChunkType::Entities => Chunk::Entities(Entities::decode(reader, ())?),
            ChunkType::Entity => Chunk::Entity(Entity::decode(reader, ())?),
            ChunkType::SkinObj => Chunk::SkinObj(Clump::decode(reader, ())?),
            ChunkType::AnimLib => Chunk::AnimLib(AnimationDictionary::decode(reader, ())?),
            ChunkType::Animation => Chunk::Animation(Clips::decode(reader, ())?),
            ChunkType::AnimationKey => Chunk::AnimationKey(AnimationKey::decode(reader, ())?),
            ChunkType::Zones => Chunk::Zones(Zones::decode(reader, (&chunk_header, world.unwrap()))?),
            ChunkType::SpLights => Chunk::SpLights(SwitchableLights::decode(reader, ())?),
            ChunkType::Collision => Chunk::Collision(Collision::decode(reader, ())?),
            ChunkType::NavigationMesh => Chunk::NavigationMesh(NavigationMesh::decode(reader, ())?),
        })
    }
}

#[derive(Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(i32)]
pub enum ChunkType {
    GLProject = 1,
    MaterialObj = 5,
    ModelGroup = 1000,
    BoneObj = 1001,
    SPMesh = 1002,
    Collision = 1003,
    AtomicMesh = 1004,
    SkinObj = 1005,
    GLCamera = 1006,
    LightObj = 1007,
    LevelObj = 1009,
    Materials = 1010,
    SectorOctree = 1011,
    World = 1012,
    AnimationKey = 1015,
    AnimLib = 1017,
    OcclusionMesh = 1018,
    Occlusion = 1019,
    WpPoints = 1020,
    NavigationMesh = 1021,
    Zones = 1023,
    Area = 1024,
    LinkEmm = 1026,
    Animation = 1027,
    SpLights = 1029,
    Entities = 20000,
    Entity = 20001,
    Textures = 20002,
}

#[derive(Debug)]
pub struct ChunkHeader {
    chunk_type: ChunkType,
    size: i32,
    version: i32,
}

impl ChunkHeader {
    pub fn get_chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }
}

impl Decode for ChunkHeader {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let chunk_type = ChunkType::try_from(i32::decode(reader, ())?)?;
        let size = {
            let size = i32::decode(reader, ())?;

            assert!(size >= 0);

            size
        };
        let version = i32::decode(reader, ())?;

        Ok(ChunkHeader {
            chunk_type,
            size,
            version,
        })
    }
}
