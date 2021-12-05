use std::{io::{Read, self}, error::Error, slice, mem};

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Bytes;
use flate2::read::GzDecoder;
use num_enum::TryFromPrimitive;

const HASH_VALUES: [u32; 256] = [
    0, 0x4C11DB7, 0x9823B6E, 0x0D4326D9, 0x130476DC,
    0x17C56B6B, 0x1A864DB2, 0x1E475005, 0x2608EDB8, 0x22C9F00F,
    0x2F8AD6D6, 0x2B4BCB61, 0x350C9B64, 0x31CD86D3, 0x3C8EA00A,
    0x384FBDBD, 0x4C11DB70, 0x48D0C6C7, 0x4593E01E, 0x4152FDA9,
    0x5F15ADAC, 0x5BD4B01B, 0x569796C2, 0x52568B75, 0x6A1936C8,
    0x6ED82B7F, 0x639B0DA6, 0x675A1011, 0x791D4014, 0x7DDC5DA3,
    0x709F7B7A, 0x745E66CD, 0x9823B6E0, 0x9CE2AB57, 0x91A18D8E,
    0x95609039, 0x8B27C03C, 0x8FE6DD8B, 0x82A5FB52, 0x8664E6E5,
    0x0BE2B5B58, 0x0BAEA46EF, 0x0B7A96036, 0x0B3687D81, 0x0AD2F2D84,
    0x0A9EE3033, 0x0A4AD16EA, 0x0A06C0B5D, 0x0D4326D90, 0x0D0F37027,
    0x0DDB056FE, 0x0D9714B49, 0x0C7361B4C, 0x0C3F706FB, 0x0CEB42022,
    0x0CA753D95, 0x0F23A8028, 0x0F6FB9D9F, 0x0FBB8BB46, 0x0FF79A6F1,
    0x0E13EF6F4, 0x0E5FFEB43, 0x0E8BCCD9A, 0x0EC7DD02D, 0x34867077,
    0x30476DC0, 0x3D044B19, 0x39C556AE, 0x278206AB, 0x23431B1C,
    0x2E003DC5, 0x2AC12072, 0x128E9DCF, 0x164F8078, 0x1B0CA6A1,
    0x1FCDBB16, 0x18AEB13, 0x54BF6A4, 0x808D07D, 0x0CC9CDCA,
    0x7897AB07, 0x7C56B6B0, 0x71159069, 0x75D48DDE, 0x6B93DDDB,
    0x6F52C06C, 0x6211E6B5, 0x66D0FB02, 0x5E9F46BF, 0x5A5E5B08,
    0x571D7DD1, 0x53DC6066, 0x4D9B3063, 0x495A2DD4, 0x44190B0D,
    0x40D816BA, 0x0ACA5C697, 0x0A864DB20, 0x0A527FDF9, 0x0A1E6E04E,
    0x0BFA1B04B, 0x0BB60ADFC, 0x0B6238B25, 0x0B2E29692, 0x8AAD2B2F,
    0x8E6C3698, 0x832F1041, 0x87EE0DF6, 0x99A95DF3, 0x9D684044,
    0x902B669D, 0x94EA7B2A, 0x0E0B41DE7, 0x0E4750050, 0x0E9362689,
    0x0EDF73B3E, 0x0F3B06B3B, 0x0F771768C, 0x0FA325055, 0x0FEF34DE2,
    0x0C6BCF05F, 0x0C27DEDE8, 0x0CF3ECB31, 0x0CBFFD686, 0x0D5B88683,
    0x0D1799B34, 0x0DC3ABDED, 0x0D8FBA05A, 0x690CE0EE, 0x6DCDFD59,
    0x608EDB80, 0x644FC637, 0x7A089632, 0x7EC98B85, 0x738AAD5C,
    0x774BB0EB, 0x4F040D56, 0x4BC510E1, 0x46863638, 0x42472B8F,
    0x5C007B8A, 0x58C1663D, 0x558240E4, 0x51435D53, 0x251D3B9E,
    0x21DC2629, 0x2C9F00F0, 0x285E1D47, 0x36194D42, 0x32D850F5,
    0x3F9B762C, 0x3B5A6B9B, 0x315D626, 0x7D4CB91, 0x0A97ED48,
    0x0E56F0FF, 0x1011A0FA, 0x14D0BD4D, 0x19939B94, 0x1D528623,
    0x0F12F560E, 0x0F5EE4BB9, 0x0F8AD6D60, 0x0FC6C70D7, 0x0E22B20D2,
    0x0E6EA3D65, 0x0EBA91BBC, 0x0EF68060B, 0x0D727BBB6, 0x0D3E6A601,
    0x0DEA580D8, 0x0DA649D6F, 0x0C423CD6A, 0x0C0E2D0DD, 0x0CDA1F604,
    0x0C960EBB3, 0x0BD3E8D7E, 0x0B9FF90C9, 0x0B4BCB610, 0x0B07DABA7,
    0x0AE3AFBA2, 0x0AAFBE615, 0x0A7B8C0CC, 0x0A379DD7B, 0x9B3660C6,
    0x9FF77D71, 0x92B45BA8, 0x9675461F, 0x8832161A, 0x8CF30BAD,
    0x81B02D74, 0x857130C3, 0x5D8A9099, 0x594B8D2E, 0x5408ABF7,
    0x50C9B640, 0x4E8EE645, 0x4A4FFBF2, 0x470CDD2B, 0x43CDC09C,
    0x7B827D21, 0x7F436096, 0x7200464F, 0x76C15BF8, 0x68860BFD,
    0x6C47164A, 0x61043093, 0x65C52D24, 0x119B4BE9, 0x155A565E,
    0x18197087, 0x1CD86D30, 0x29F3D35, 0x65E2082, 0x0B1D065B,
    0x0FDC1BEC, 0x3793A651, 0x3352BBE6, 0x3E119D3F, 0x3AD08088,
    0x2497D08D, 0x2056CD3A, 0x2D15EBE3, 0x29D4F654, 0x0C5A92679,
    0x0C1683BCE, 0x0CC2B1D17, 0x0C8EA00A0, 0x0D6AD50A5, 0x0D26C4D12,
    0x0DF2F6BCB, 0x0DBEE767C, 0x0E3A1CBC1, 0x0E760D676, 0x0EA23F0AF,
    0x0EEE2ED18, 0x0F0A5BD1D, 0x0F464A0AA, 0x0F9278673, 0x0FDE69BC4,
    0x89B8FD09, 0x8D79E0BE, 0x803AC667, 0x84FBDBD0, 0x9ABC8BD5,
    0x9E7D9662, 0x933EB0BB, 0x97FFAD0C, 0x0AFB010B1, 0x0AB710D06,
    0x0A6322BDF, 0x0A2F33668, 0x0BCB4666D, 0x0B8757BDA, 0x0B5365D03,
    0x0B1F740B4
];

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

					for _ in 0 .. textures_count {
                        let texture = Texture::decode(&mut decoder)?;

						textures.push(texture);
					}
				},
                ChunkType::Materials => {
                    let materials_count = decoder.read_i32::<LittleEndian>()?;
                },
                ChunkType::MaterialObj => {
                    let material = Material::decode(&mut decoder)?;

                    //println!("{:X}", material.get_hash());
                },
				_ => {
					decoder
						.read_exact(
							{
								let mut buffer = Vec::new();
								buffer.resize(chunk_header.get_size() as usize, 0u8);
								buffer
							}
							.as_mut(),
						)
						.unwrap();
				}
			}
		}

		Ok(())
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
pub enum ChunkType {
    Textures = 20002,
    Materials = 1010,
    MaterialObj = 5,
    World = 1012,
    AnimLib = 1017,
    Entities = 20000,
    Entity = 20001,
    SpLights = 1029,
    Zones = 1023,
    NavigationMesh = 1021,
    WpPoints = 1020,
    SectorOctree = 1011,
    Occlusion = 1019,
    Area = 1024,
    SkinObj = 1005,
    BoneObj = 1001,
    OcclusionMesh = 1018,
    ModelGroup = 1000,
    SPMesh = 1002,
    Collision = 1003,
    AtomicMesh = 1004,
    GLCamera = 1006,
    GLProject = 1,
    LightObj = 1007,
    LinkEmm = 1026,
    LevelObj = 1009,
}

#[derive(Debug)]
pub struct ChunkHeader {
    chunk_type: ChunkType,
    size: i32,
    version: i32,
}

impl ChunkHeader {
    pub fn decode(reader: &mut impl Read) -> Result<ChunkHeader, Box<dyn Error>> {
        Ok(ChunkHeader {
            chunk_type: ChunkType::try_from(reader.read_i32::<LittleEndian>()?)?,
            size: reader.read_i32::<LittleEndian>()?,
            version: reader.read_i32::<LittleEndian>()?,
        })
    }

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

pub struct Texture {
    name: String,
    mask_name: String,
    width: i32,
    height: i32,
    filter: i32,
    address: i32,
    format: i32,
    border_color: Rgba,
    pixels: Vec<Rgba>,
}

impl Texture {
    pub fn decode(reader: &mut impl Read) -> io::Result<Texture> {
        let name_length = reader.read_i32::<LittleEndian>()?;

        let mut name = Vec::with_capacity(name_length as usize);

        for _ in 0..name_length {
            name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
        }

        let name = name.iter().collect::<String>();

        let mask_name_length = reader.read_i32::<LittleEndian>()?;

        let mut mask_name = Vec::with_capacity(mask_name_length as usize);

        for _ in 0..mask_name_length {
            mask_name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
        }

        let mask_name = mask_name.into_iter().collect::<String>();

        let width = reader.read_i32::<LittleEndian>()?;
        let height = reader.read_i32::<LittleEndian>()?;
        let filter = reader.read_i32::<LittleEndian>()?;
        let address = reader.read_i32::<LittleEndian>()?;
        let format = reader.read_i32::<LittleEndian>()?;

        let border_color = Rgba::decode(reader)?;

        let mut pixels = Vec::with_capacity((width * height) as usize);

        for _ in 0..width * height {
            pixels.push(Rgba::decode(reader)?);
        }

        Ok(Texture {
            name,
            mask_name,
            width,
            height,
            filter,
            address,
            format,
            border_color,
            pixels,
        })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_mask_name(&self) -> &String {
        &self.mask_name
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_filter(&self) -> i32 {
        self.filter
    }

    pub fn get_address(&self) -> i32 {
        self.address
    }

    pub fn get_format(&self) -> i32 {
        self.format
    }

    pub fn get_border_color(&self) -> &Rgba {
        &self.border_color
    }

    pub fn get_pixels(&self) -> &Vec<Rgba> {
        &self.pixels
    }

    pub fn get_hash(&self) -> u32 {
        let data = [self.address, self.filter];

        hash(unsafe { slice::from_raw_parts(self.pixels.as_ptr() as *const _ as *const u8, (self.width * self.height * 4) as usize) }) ^ hash(unsafe { slice::from_raw_parts(data.as_ptr() as *const _ as *const u8, 8)})
    }
}

struct Material {
    material_hash: u32,
    attributes: Attributes,
    textures: [MaterialTexture; 5],
}

impl Material {
    fn decode(reader: &mut impl Read) -> io::Result<Material> {
        let flags = reader.read_u32::<LittleEndian>()?;
        let _name_hash = reader.read_u32::<LittleEndian>()?;
        let additive_lighting_model = reader.read_i32::<LittleEndian>()? != 0;
        let colour = Rgba::decode(reader)?;
        let specular = Rgba::decode(reader)?;
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

        for i in 0 .. 5 {
            let uv_set = reader.read_u32::<LittleEndian>()?;
            //println!("UV set: {}", uv_set);
            let name_length = reader.read_i32::<LittleEndian>()?;
            //println!("Name length: {}", name_length);
            if name_length <= 0 {
                continue;
            }
            let mut name = Vec::with_capacity(name_length as usize);
            for _ in 0..name_length {
                name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
            }
            let name = name.iter().collect::<String>();
            let format = reader.read_i32::<LittleEndian>()?;
            let filter = reader.read_i32::<LittleEndian>()?;
            let address = reader.read_i32::<LittleEndian>()?;
            let mask_name_length = reader.read_i32::<LittleEndian>()?;
            let mut mask_name = Vec::with_capacity(mask_name_length as usize);
            for _ in 0..mask_name_length {
                mask_name.push(reader.read_i32::<LittleEndian>()? as u8 as char);
            }
            let mask_name = mask_name.into_iter().collect::<String>();
            let border_colour = Rgba::decode(reader)?;
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

        for i in 0 .. 5 {
            let use_matrix = reader.read_i32::<LittleEndian>()? != 0;

            if use_matrix {
                let matrix = Matrix::decode(reader)?;

                matrices[i] = Some(matrix);
            }

            use_matrices[i] = use_matrix;
        }

        for i in 0 .. 5 {
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

    fn get_hash(&self) -> u32 {
        hash(unsafe { slice::from_raw_parts(mem::transmute(&self), mem::size_of::<Attributes>()) })
    }
}

#[repr(C)]
#[derive(Default)]
struct Attributes {
    flags: u32,
    additive_lighting_model: bool,
    colour: Rgba,
    specular: Rgba,
    power: f32,
    shading_mode: i32,
    depth_buffer_write: bool,
    depth_buffer_comparison_mode: i32,
    blend: bool,
    blend_modes: BlendModes,
    alpha_test: bool,
    alpha_test_mode: AlphaTestMode,
    owner: u32,
    colour_buffer_write: u32,
    use_matrices: [bool; 5],
    generators: [i32; 5],
    uv_sets: [u32; 5],
    texture_hashes: [u32; 5],
    envmap_type: i32,
    planar_sheer_envmap_distance: f32,
}

struct Matrix {
    right: Vector4,
    up: Vector4,
    at: Vector4,
    position: Vector4,
    flags: u64,
}

impl Matrix {
    fn decode(reader: &mut impl Read) -> io::Result<Matrix> {
        let right = Vector3::decode(reader)?;
        let right = Vector4::new(right.x, right.y, right.z, 0.0);
        let up = Vector3::decode(reader)?;
        let up = Vector4::new(up.x, up.y, up.z, 0.0);
        let at = Vector3::decode(reader)?;
        let at = Vector4::new(at.x, at.y, at.z, 0.0);
        let position = Vector3::decode(reader)?;
        let position = Vector4::new(position.x, position.y, position.z, 1.0);
        let flags = reader.read_u64::<LittleEndian>()?;

        Ok(Matrix {
            right,
            up,
            at,
            position,
            flags
        })
    }
}

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    fn decode(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            x: reader.read_f32::<LittleEndian>()?,
            y: reader.read_f32::<LittleEndian>()?,
            z: reader.read_f32::<LittleEndian>()?, 
        })
    }
}

struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4 {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w,
        }
    }
}

#[derive(Default)]
struct MaterialTexture {
    uv_set: u32,
    name: String,
    format: i32,
    address: i32,
    mask_name: String,
    border_colour: Rgba,
    hash: u32,
}

#[derive(Default, Debug)]
struct BlendModes {
    source_mode: i32,
    destionation_mode: i32,
}

impl BlendModes {
    fn decode(reader: &mut impl Read) -> io::Result<BlendModes> {
        Ok(BlendModes {
            source_mode: reader.read_i32::<LittleEndian>()?,
            destionation_mode: reader.read_i32::<LittleEndian>()?,
        })
    }
}

#[derive(Default, Debug)]
struct AlphaTestMode {
    comparision_function: i32,
    reference: f32,
}

impl AlphaTestMode {
    fn decode(reader: &mut impl Read) -> io::Result<AlphaTestMode> {
        Ok(AlphaTestMode {
            comparision_function: reader.read_i32::<LittleEndian>()?,
            reference: reader.read_f32::<LittleEndian>()?,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn decode(reader: &mut impl Read) -> io::Result<Rgba> {
        Ok(Rgba {
            r: reader.read_i32::<LittleEndian>()? as u8,
            g: reader.read_i32::<LittleEndian>()? as u8,
            b: reader.read_i32::<LittleEndian>()? as u8,
            a: reader.read_i32::<LittleEndian>()? as u8,
        })
    }
}

fn hash(values: &[u8]) -> u32 {
    let mut hash = 0u32;

    for value in values {
        hash = HASH_VALUES[((hash >> 24) ^ (*value as u32)) as usize] ^ (hash << 8);
    }

    (values.len() as u32) ^ hash
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn hash_test() {
        assert_eq!(hash(&['M' as u8, 'D' as u8, 'L' as u8, '-' as u8, 'G' as u8, 'O' as u8, 'D' as u8]), 0x5170CFB0);
    }

    #[test]
    fn decode_file_test() {
        let decoder = BspDecoder::new(Bytes::copy_from_slice(&fs::read("Darkling.bsp").unwrap()));

        decoder.decode().unwrap();
    }
}
