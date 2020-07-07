use std::mem;

pub struct FrameBuffer {
    frame: Vec<char>,
    width: u16,
    height: u16,
}

impl FrameBuffer {
    pub fn create(width: u16, height: u16) -> Self {
        let mut frame = Vec::with_capacity((width * height).into());
        for indexes in 0..(width * height) {
            frame.push(' '); // empty symbol...
        }
        FrameBuffer { frame, width, height }
    }

    pub fn push_fb(&mut self, mut frame: Vec<char>) {
        mem::swap(&mut frame, &mut self.frame);
    }

    pub fn get(&mut self, x: u16, y: u16) -> char {
        self.frame[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, text: &str, x: u16, y: u16) {
        for (i, v) in text.chars().enumerate() {
            self.frame[(y * self.width + (x + i as u16)) as usize] = v;
        }
    }
}