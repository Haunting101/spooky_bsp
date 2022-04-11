#![feature(associated_type_defaults)]

pub(crate) mod hash;

mod algebra;
mod bounding_box;
mod bsp;
mod chunk;
mod color;
mod decode;
mod string;

pub use algebra::*;
pub use bounding_box::*;
pub use bsp::*;
pub use chunk::*;
pub use color::*;
pub use decode::*;
pub use string::*;

use std::io::{self, Read};

pub struct PositionTracker<R: Read> {
    reader: R,
    current_position: usize,
}

impl<R: Read> PositionTracker<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.current_position
    }
}

impl<R: Read> Read for PositionTracker<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let read_bytes = self.reader.read(buffer)?;

        self.current_position += read_bytes;

        Ok(read_bytes)
    }
}
