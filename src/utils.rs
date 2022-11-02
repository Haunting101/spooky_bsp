use std::{
    cmp::min,
    collections::VecDeque,
    io::{self, Read},
};

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

pub struct PeekableReader<R: Read> {
    buffer: VecDeque<u8>,
    reader: R,
}

impl<R: Read> PeekableReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            buffer: VecDeque::new(),
            reader,
        }
    }

    pub fn peek<const N: usize>(&mut self) -> Result<[u8; N], io::Error> {
        if self.buffer.len() < N {
            let mut temporary_buffer = vec![0; N - self.buffer.len()];

            self.reader.read_exact(&mut temporary_buffer)?;
            self.buffer.append(&mut VecDeque::from(temporary_buffer));
        }

        let mut values = [0; N];

        for i in 0..N {
            values[i] = self.buffer[i];
        }

        Ok(values)
    }
}

impl<R: Read> Read for PeekableReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.buffer.len() == 0 {
            Ok(self.reader.read(buffer)?)
        } else {
            let len = min(self.buffer.len(), buffer.len());

            for i in 0..len {
                buffer[i] = self.buffer.pop_front().unwrap();
            }

            Ok(len)
        }
    }
}
