use derive_new::new;
use std::io::Read;

use crate::Decode;

#[derive(new)]
pub struct Entities {
    pub count: u32,
}

impl Decode for Entities {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let count = u32::decode(reader, ())?;

        Ok(Self::new(count))
    }
}
