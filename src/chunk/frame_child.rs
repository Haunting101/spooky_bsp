use derive_new::new;
use std::io::Read;

use crate::Decode;

#[derive(new, Clone, Debug)]
pub struct FrameChild {
    pub stream_depth: u32,
}

impl Decode for FrameChild {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let stream_depth = u32::decode(reader, ())?;

        Ok(Self::new(stream_depth))
    }
}
