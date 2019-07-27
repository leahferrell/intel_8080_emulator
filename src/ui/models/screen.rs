const HEIGHT: usize = 256;
const WIDTH: usize = 224;

pub struct Screen {
    pub pixels: [[u8; HEIGHT]; WIDTH],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [[0; HEIGHT]; WIDTH],
        }
    }

    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.pixels[ind[1]][ind[0]] = val;
    }
}