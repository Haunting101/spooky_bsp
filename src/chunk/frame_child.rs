use crate::Decode;

#[derive(Clone, Debug, Decode)]
pub struct FrameChild {
    pub stream_depth: u32,
}
