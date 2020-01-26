use image::{ImageBuffer, GrayImage, FilterType, DynamicImage};

#[derive(Default)]
pub struct Canvas {
    pad: usize,
    width: usize,
    height: usize,
    text: String,
    char_height: usize,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        let mut new = Self::default();
        new.width = w;
        new.height = h;
        new
    }

    pub fn pad(&mut self, pad: usize) -> &mut Self {
        self.pad = pad;
        self
    }

    pub fn text<S>(&mut self, text: S) -> &mut Self
    where
        S: ToString,
    {
        self.text = text.to_string();
        self
    }

    pub fn build(&mut self) {
        self.char_height = self.height - (self.pad * 2);
    }

    pub fn get_image(&self) -> GrayImage {
        let mut blank = DynamicImage::new_luma8(self.width as u32, self.height as u32);
        blank.invert();
        blank.to_luma()
    }
}
