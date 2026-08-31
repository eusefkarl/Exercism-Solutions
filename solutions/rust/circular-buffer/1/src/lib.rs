use std::iter::repeat_with;

pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    read_pos: usize,
    write_pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: repeat_with(|| None)
        .take(capacity)
        .collect(),
            capacity,
            write_pos: 0,
            read_pos: 0
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.buffer[self.write_pos].is_some() {
        return Err(Error::FullBuffer);
        }
        self.buffer[self.write_pos] = Some(_element);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer[self.read_pos].is_none() {
        return Err(Error::EmptyBuffer);
        }
        let val = self.buffer[self.read_pos].take().unwrap();
        self.read_pos = (self.read_pos + 1) % self.capacity;
        Ok(val)
        
    }

    pub fn clear(&mut self) {
        self.write_pos = 0;
        self.read_pos = 0;
        for buffer in &mut self.buffer {
            *buffer = None;
        }
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.buffer[self.write_pos].is_some() {
            self.buffer[self.read_pos] = Some(_element);
            self.read_pos = (self.read_pos + 1) % self.capacity;
        }
        else{
            self.buffer[self.write_pos] = Some(_element);
            
        }
        self.write_pos = (self.write_pos + 1) % self.capacity;
    }
}
