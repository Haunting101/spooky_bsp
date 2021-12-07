use std::{error::Error, io::Read};

use crate::{ChunkHeader, ChunkType, Material, Texture};
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

    pub fn decode(&self) -> Result<(), Box<dyn Error>> {
        let mut decoder = GzDecoder::new(self.bytes.as_ref());

        while let Ok(chunk_header) = ChunkHeader::decode(&mut decoder) {
            //println!("{:?}", chunk_header);

            match chunk_header.get_chunk_type() {
                ChunkType::Textures => {
                    let textures_count = decoder.read_i32::<LittleEndian>()?;
                    let mut textures = Vec::with_capacity(textures_count as usize);

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

                    //println!("{:X}", material.get_hash());
                }
                _ => decoder.read_exact(vec![0u8; chunk_header.get_size() as usize].as_mut())?,
            }
        }

        Ok(())
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
