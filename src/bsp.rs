use crate::{
    AnimationDictionary, AnimationKey, AtomicMesh, CameraProjection, ChunkHeader, ChunkType, Clips,
    Clump, Collision, Decode, Entities, Entity, Frame, FrameChild, Light, Material, Mesh,
    ModelPart, NGonList, NavigationMesh, NullBox, Nulls, Occlusion, PositionTracker, SectorOctree,
    Spline, SwitchableLights, Texture, World, Zones,
};
use derive_new::new;
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(new)]
pub struct Bsp {
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub world: Option<World>,
    pub meshes: Vec<Mesh>,
    pub model_parts: Vec<ModelPart>,
    pub octree_sectors: Vec<SectorOctree>,
    pub occlusions: Vec<Occlusion>,
    pub frame_children: Vec<FrameChild>,
    pub null_boxes: Vec<NullBox>,
    pub atomic_meshes: Vec<AtomicMesh>,
    pub camera_projections: Vec<CameraProjection>,
    pub lights: Vec<Light>,
    pub ngon_lists: Vec<NGonList>,
    pub splines: Vec<Spline>,
    pub frames: Vec<Frame>,
    pub nullss: Vec<Nulls>,
    pub entitiess: Vec<Entities>,
    pub entities: Vec<Entity>,
    pub clumps: Vec<Clump>,
    pub animation_dictionaries: Vec<AnimationDictionary>,
    pub clips: Vec<Clips>,
    pub animation_keys: Vec<AnimationKey>,
    pub zones: Vec<Zones>,
    pub switchable_lights: Vec<SwitchableLights>,
    pub collisions: Vec<Collision>,
    pub navigation_meshes: Vec<NavigationMesh>,
}

impl Decode for Bsp {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let mut decoder = PositionTracker::new(GzDecoder::new(reader));

        let mut textures = Vec::new();
        let mut materials = Vec::new();
        let mut world = None;
        let mut meshes = Vec::new();
        let mut model_parts = Vec::new();
        let mut octree_sectors = Vec::new();
        let mut occlusions = Vec::new();
        let mut frame_children = Vec::new();
        let mut null_boxes = Vec::new();
        let mut atomic_meshes = Vec::new();
        let mut camera_projections = Vec::new();
        let mut lights = Vec::new();
        let mut ngon_lists = Vec::new();
        let mut splines = Vec::new();
        let mut frames = Vec::new();
        let mut nullss = Vec::new();
        let mut entitiess = Vec::new();
        let mut entities = Vec::new();
        let mut clumps = Vec::new();
        let mut animation_dictionaries = Vec::new();
        let mut clips = Vec::new();
        let mut animation_keys = Vec::new();
        let mut zones = Vec::new();
        let mut switchable_lights = Vec::new();
        let mut collisions = Vec::new();
        let mut navigation_meshes = Vec::new();

        let mut material_count = 0;

        while let Ok(chunk_header) = ChunkHeader::decode(&mut decoder, ()) {
            let previous_position = decoder.position();

            match chunk_header.get_chunk_type() {
                ChunkType::Textures => textures = Vec::decode(&mut decoder, ())?,
                ChunkType::Materials => {
                    material_count = i32::decode(&mut decoder, ())?;

                    assert!(material_count >= 0);
                }
                ChunkType::MaterialObj => materials.push(Material::decode(&mut decoder, ())?),
                ChunkType::World => {
                    world = Some(World::decode(&mut decoder, ())?);
                }
                ChunkType::ModelGroup => meshes.push(Mesh::decode(&mut decoder, ())?),
                ChunkType::SPMesh => model_parts.push(ModelPart::decode(&mut decoder, ())?),
                ChunkType::SectorOctree => {
                    octree_sectors.push(SectorOctree::decode(&mut decoder, ())?)
                }
                ChunkType::Occlusion => occlusions.push(Occlusion::decode(&mut decoder, ())?),
                ChunkType::LevelObj => {
                    frame_children.push(FrameChild::decode(&mut decoder, ())?);
                }
                ChunkType::LinkEmm => {
                    null_boxes.push(NullBox::decode(&mut decoder, ())?);
                }
                ChunkType::AtomicMesh => {
                    atomic_meshes.push(AtomicMesh::decode(&mut decoder, ())?);
                }
                ChunkType::GLCamera => {
                    camera_projections.push(CameraProjection::decode(&mut decoder, ())?);
                }
                ChunkType::GLProject => {
                    camera_projections.push(CameraProjection::decode(&mut decoder, ())?);
                }
                ChunkType::LightObj => {
                    lights.push(Light::decode(&mut decoder, &chunk_header)?);
                }
                ChunkType::OcclusionMesh => {
                    ngon_lists.push(NGonList::decode(&mut decoder, ())?);
                }
                ChunkType::Area => {
                    splines.push(Spline::decode(&mut decoder, ())?);
                }
                ChunkType::BoneObj => {
                    frames.push(Frame::decode(&mut decoder, ())?);
                }
                ChunkType::WpPoints => {
                    nullss.push(Nulls::decode(&mut decoder, ())?);
                }
                ChunkType::Entities => {
                    entitiess.push(Entities::decode(&mut decoder, ())?);
                }
                ChunkType::Entity => {
                    entities.push(Entity::decode(&mut decoder, ())?);
                }
                ChunkType::SkinObj => {
                    clumps.push(Clump::decode(&mut decoder, ())?);
                }
                ChunkType::AnimLib => {
                    animation_dictionaries.push(AnimationDictionary::decode(&mut decoder, ())?);
                }
                ChunkType::Animation => {
                    clips.push(Clips::decode(&mut decoder, ())?);
                }
                ChunkType::AnimationKey => {
                    animation_keys.push(AnimationKey::decode(&mut decoder, ())?);
                }
                ChunkType::Zones => {
                    zones.push(Zones::decode(&mut decoder, (&chunk_header, world.as_ref().unwrap()))?);
                }
                ChunkType::SpLights => {
                    switchable_lights.push(SwitchableLights::decode(&mut decoder, ())?);
                }
                ChunkType::Collision => {
                    collisions.push(Collision::decode(&mut decoder, ())?);
                }
                ChunkType::NavigationMesh => {
                    navigation_meshes.push(NavigationMesh::decode(&mut decoder, ())?);
                }
            }

            let current_position = decoder.position();
            let read_bytes = current_position - previous_position;

            assert_eq!(read_bytes, chunk_header.get_size() as usize);
        }

        assert!(material_count as usize == materials.len());

        Ok(Bsp::new(
            textures,
            materials,
            world,
            meshes,
            model_parts,
            octree_sectors,
            occlusions,
            frame_children,
            null_boxes,
            atomic_meshes,
            camera_projections,
            lights,
            ngon_lists,
            splines,
            frames,
            nullss,
            entitiess,
            entities,
            clumps,
            animation_dictionaries,
            clips,
            animation_keys,
            zones,
            switchable_lights,
            collisions,
            navigation_meshes,
        ))
    }
}

#[cfg(test)]
mod tests {
    mod ghosts {
        use crate::{Bsp, Decode};
        use claim::assert_ok;
        use std::fs::File;
        use test_case::test_case;

        use super::*;

        #[test_case("aether.bsp" ; "aether")]
        #[test_case("arclight.bsp" ; "arclight")]
        #[test_case("azrael.bsp" ; "azrael")]
        #[test_case("banzai.bsp" ; "banzai")]
        #[test_case("BlackCrow.bsp" ; "blackcrow")]
        #[test_case("BlairWisp.bsp" ; "blairwisp")]
        #[test_case("bluemurder.bsp" ; "bluemurder")]
        #[test_case("boo.bsp" ; "boo")]
        #[test_case("Buck.bsp" ; "buck")]
        #[test_case("Carter.bsp" ; "carter")]
        #[test_case("Clatterclaws.bsp" ; "clatterclaws")]
        #[test_case("cogjammer.bsp" ; "cogjammer")]
        #[test_case("Darkling.bsp" ; "darkling")]
        #[test_case("daydreamer.bsp" ; "daydreamer")]
        #[test_case("Dragoon.bsp" ; "dragoon")]
        #[test_case("Electrospasm.bsp" ; "electrospasm")]
        #[test_case("Fingers.bsp" ; "fingers")]
        #[test_case("firetail.bsp" ; "firetail")]
        #[test_case("FlashJordan.bsp" ; "flashjordan")]
        #[test_case("ghastly.bsp" ; "ghastly")]
        #[test_case("harriet.bsp" ; "harriet")]
        #[test_case("hogwash.bsp" ; "hogwash")]
        #[test_case("hypnos.bsp" ; "hypnos")]
        #[test_case("Knuckles.bsp" ; "knuckles")]
        #[test_case("ladyrose.bsp" ; "ladyrose")]
        #[test_case("Lucky.bsp" ; "lucky")]
        #[test_case("maxfactor.bsp" ; "maxfactor")]
        #[test_case("Mirage.bsp" ; "mirage")]
        #[test_case("moonscream.bsp" ; "moonscream")]
        #[test_case("Painter.bsp" ; "painter")]
        #[test_case("Quiver.bsp" ; "quiver")]
        #[test_case("raindancer.bsp" ; "raindancer")]
        #[test_case("scarecrow.bsp" ; "scarecrow")]
        #[test_case("shivers.bsp" ; "shivers")]
        #[test_case("SmokinJoe.bsp" ; "smokinjoe")]
        #[test_case("soulscreech.bsp" ; "soulscreech")]
        #[test_case("sparkle.bsp" ; "sparkle")]
        #[test_case("stonewall.bsp" ; "stonewall")]
        #[test_case("stormtalon.bsp" ; "stormtalon")]
        #[test_case("TerrorEyes.bsp" ; "terroreyes")]
        #[test_case("Thorne.bsp" ; "thorne")]
        #[test_case("wavemaster.bsp" ; "wavemaster")]
        #[test_case("weatherwitch.bsp" ; "weatherwitch")]
        #[test_case("Wendel.bsp" ; "wendel")]
        #[test_case("Whisperwind.bsp" ; "whisperwind")]
        #[test_case("Wily.bsp" ; "wily")]
        #[test_case("windwalker.bsp" ; "windwalker")]
        fn decode_file(asset: &str) {
            assert_ok!(Bsp::decode(
                &mut File::open(format!("assets/ghosts/{}", asset)).unwrap(),
                (),
            ));
        }

        mod animations {
            use claim::assert_ok;
            use std::fs::File;
            use test_case::test_case;

            use super::*;

            #[test_case("aether_anims" ; "aether_anims")]
            #[test_case("arclight_anims" ; "arclight_anims")]
            #[test_case("azrael_anims" ; "azrael_anims")]
            #[test_case("banzai_anims" ; "banzai_anims")]
            #[test_case("blackcrow_anims" ; "blackcrow_anims")]
            #[test_case("blairwisp_anims" ; "blairwisp_anims")]
            #[test_case("bluemurder_anims" ; "bluemurder_anims")]
            #[test_case("boo_anims" ; "boo_anims")]
            #[test_case("brigit_anims" ; "brigit_anims")]
            #[test_case("buck_anims" ; "buck_anims")]
            #[test_case("carter_anims" ; "carter_anims")]
            #[test_case("clatterclaws_anims" ; "clatterclaws_anims")]
            #[test_case("cogjammer_anims" ; "cogjammer_anims")]
            #[test_case("darkling_anims" ; "darkling_anims")]
            #[test_case("daydreamer_anims" ; "daydreamer_anims")]
            #[test_case("dragoon_anims" ; "dragoon_anims")]
            #[test_case("electrospasm_anims" ; "electrospasm_anims")]
            #[test_case("fingers_anims" ; "fingers_anims")]
            #[test_case("firetail_anims" ; "firetail_anims")]
            #[test_case("flashjordan_anims" ; "flashjordan_anims")]
            #[test_case("ghastly_anims" ; "ghastly_anims")]
            #[test_case("hardboiled_anims" ; "hardboiled_anims")]
            #[test_case("harriet_anims" ; "harriet_anims")]
            #[test_case("hogwash_anims" ; "hogwash_anims")]
            #[test_case("hypnos_anims" ; "hypnos_anims")]
            #[test_case("knuckles_anims" ; "knuckles_anims")]
            #[test_case("ladyrose_anims" ; "ladyrose_anims")]
            #[test_case("lucky_anims" ; "lucky_anims")]
            #[test_case("maxfactor_anims" ; "maxfactor_anims")]
            #[test_case("mirage_anims" ; "mirage_anims")]
            #[test_case("moonscream_anims" ; "moonscream_anims")]
            #[test_case("painter_anims" ; "painter_anims")]
            #[test_case("quiver_anims" ; "quiver_anims")]
            #[test_case("raindancer_anims" ; "raindancer_anims")]
            #[test_case("scarecrow_anims" ; "scarecrow_anims")]
            #[test_case("shivers_anims" ; "shivers_anims")]
            #[test_case("smokinjoe_anims" ; "smokinjoe_anims")]
            #[test_case("soulscreech_anims" ; "soulscreech_anims")]
            #[test_case("sparkle_anims" ; "sparkle_anims")]
            #[test_case("static_anims" ; "static_anims")]
            #[test_case("stonewall_anims" ; "stonewall_anims")]
            #[test_case("stormtalon_anims" ; "stormtalon_anims")]
            #[test_case("terroreyes_anims" ; "terroreyes_anims")]
            #[test_case("thorne_anims" ; "thorne_anims")]
            #[test_case("wavemaster_anims" ; "wavemaster_anims")]
            #[test_case("weatherwitch_anims" ; "weatherwitch_anims")]
            #[test_case("wendel_anims" ; "wendel_anims")]
            #[test_case("whirlweird_anims" ; "whirlweird_anims")]
            #[test_case("whisperwind_anims" ; "whisperwind_anims")]
            #[test_case("wily_anims" ; "wily_anims")]
            #[test_case("windwalker_anims" ; "windwalker_anims")]
            fn decode_file(asset: &str) {
                assert_ok!(Bsp::decode(
                    &mut File::open(format!("assets/ghosts/animations/{}.bsp", asset)).unwrap(),
                    (),
                ));
            }
        }
    }

    mod scenarios {
        use crate::{Bsp, Decode};
        use claim::assert_ok;
        use std::fs::File;
        use test_case::test_case;

        use super::*;

        #[test_case("blairwisp" ; "blairwisp")]
        #[test_case("blues" ; "blues")]
        #[test_case("calamity" ; "calamity")]
        #[test_case("cuckoos_nest" ; "cuckoos_nest")]
        #[test_case("deadfellas" ; "deadfellas")]
        #[test_case("facepacks" ; "facepacks")]
        #[test_case("FINALE" ; "finale")]
        #[test_case("ghostbreakers" ; "ghostbreakers")]
        #[test_case("ghoul" ; "ghoul")]
        #[test_case("GHOULROOM" ; "ghoulroom")]
        #[test_case("haunting101" ; "haunting101")]
        #[test_case("MAP" ; "map")]
        #[test_case("mortal_jacket" ; "mortal_jacket")]
        #[test_case("poultrygeist" ; "poultrygeist")]
        #[test_case("spooky" ; "spooky")]
        #[test_case("summoners" ; "summoners")]
        #[test_case("weirdseance" ; "weirdseance")]
        fn decode_file(asset: &str) {
            assert_ok!(Bsp::decode(
                &mut File::open(format!("assets/scenarios/{}/gamedata.bsp", asset)).unwrap(),
                (),
            ));
        }
    }

    mod levels {
        use crate::{Bsp, Decode};
        use claim::assert_ok;
        use std::fs::File;
        use test_case::test_case;

        use super::*;

        #[test_case("armybase" ; "armybase")]
        #[test_case("Asylum" ; "asylum")]
        #[test_case("blair" ; "blair")]
        #[test_case("calamity" ; "calamity")]
        #[test_case("cuckoos_nest" ; "cuckoos_nest")]
        #[test_case("Deadfellas" ; "deadfellas")]
        #[test_case("Facepacks" ; "facepacks")]
        #[test_case("finale" ; "finale")]
        #[test_case("Frat" ; "frat")]
        #[test_case("ghostbreakers" ; "ghostbreakers")]
        #[test_case("ghoulroom" ; "ghoulroom")]
        #[test_case("map" ; "map")]
        #[test_case("Police" ; "police")]
        #[test_case("sorority" ; "sorority")]
        #[test_case("spooky" ; "spooky")]
        #[test_case("summoners" ; "summoners")]
        fn decode_file(asset: &str) {
            assert_ok!(Bsp::decode(
                &mut File::open(format!("assets/levels/{}.bsp", asset)).unwrap(),
                (),
            ));
        }
    }
}
