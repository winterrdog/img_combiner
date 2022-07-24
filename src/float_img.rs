use crate::errors::ImgDataErrors;

#[derive(Debug)]
pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buf_capacity = 3_655_744usize;
        let buf = Vec::<u8>::with_capacity(buf_capacity);

        Self {
            width,
            height,
            data: buf,
            name,
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImgDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImgDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}
