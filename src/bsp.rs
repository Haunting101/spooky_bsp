use crate::{Chunk, ChunkHeader, Decode, DecodeError, PeekableReader, PositionTracker};
use flate2::read::GzDecoder;
use std::io::{ErrorKind, Read};

const GZIP_MAGIC_NUMBER: [u8; 2] = [0x1f, 0x8b];

pub struct Bsp {
    pub chunks: Vec<Chunk>,
}

impl Decode for Bsp {
    fn decode(reader: &mut impl Read, _state: ()) -> Result<Self, DecodeError> {
        let mut reader: PositionTracker<Box<dyn Read>> = {
            let mut reader = PeekableReader::new(reader);

            let magic_number = reader.peek::<2>()?;

            if magic_number == GZIP_MAGIC_NUMBER {
                PositionTracker::new(Box::new(GzDecoder::new(reader)))
            } else {
                PositionTracker::new(Box::new(reader))
            }
        };

        let mut chunks = Vec::new();

        let mut latest_world = None;

        loop {
            match ChunkHeader::decode(&mut reader, ()) {
                Ok(chunk_header) => {
                    let expected_size = chunk_header.get_size() as usize;
                    let previous_position = reader.position();

                    let chunk = Chunk::decode(&mut reader, (chunk_header, latest_world.as_ref()))?;

                    match chunk {
                        Chunk::World(ref current_world) => {
                            latest_world = Some(current_world.clone());
                        }
                        _ => (),
                    }

                    chunks.push(chunk);

                    let current_position = reader.position();
                    let actual_size = current_position - previous_position;

                    if expected_size != actual_size {
                        return Err(DecodeError::ReadTooMuchData {
                            expected: expected_size,
                            actual: actual_size,
                        });
                    }
                }
                Err(error) => match error {
                    DecodeError::IO(error) if error.kind() == ErrorKind::UnexpectedEof => break,
                    _ => return Err(error),
                },
            }
        }

        Ok(Bsp { chunks })
    }
}

#[cfg(test)]
mod tests {
    mod ghosts {
        use crate::{Bsp, Decode};
        use claim::assert_ok;
        use std::{fs::File, io::BufReader};
        use test_case::test_case;

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
                &mut BufReader::new(File::open(format!("assets/ghosts/{}", asset)).unwrap()),
                (),
            ));
        }

        mod animations {
            use crate::{Bsp, Decode};
            use claim::assert_ok;
            use std::{fs::File, io::BufReader};
            use test_case::test_case;

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
            #[test_case("wavemaster_anims" ; "wavemaster_anims")]
            #[test_case("weatherwitch_anims" ; "weatherwitch_anims")]
            #[test_case("wendel_anims" ; "wendel_anims")]
            #[test_case("whisperwind_anims" ; "whisperwind_anims")]
            #[test_case("wily_anims" ; "wily_anims")]
            #[test_case("windwalker_anims" ; "windwalker_anims")]
            fn decode_file(asset: &str) {
                assert_ok!(Bsp::decode(
                    &mut BufReader::new(
                        File::open(format!("assets/ghosts/animations/{}.bsp", asset)).unwrap()
                    ),
                    (),
                ));
            }
        }
    }

    mod scenarios {
        use crate::{Bsp, Decode};
        use claim::assert_ok;
        use std::{fs::File, io::BufReader};
        use test_case::test_case;

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
                &mut BufReader::new(
                    File::open(format!("assets/scenarios/{}/gamedata.bsp", asset)).unwrap()
                ),
                (),
            ));
        }
    }

    mod levels {
        use crate::{Bsp, Decode};
        use claim::assert_ok;
        use std::{fs::File, io::BufReader};
        use test_case::test_case;

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
                &mut BufReader::new(File::open(format!("assets/levels/{}.bsp", asset)).unwrap()),
                (),
            ));
        }
    }
}
