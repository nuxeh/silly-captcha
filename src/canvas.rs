use image::{ImageBuffer, GrayImage, FilterType, DynamicImage};

use crate::character;

#[derive(Default)]
pub struct Canvas {
    pad: usize,
    width: usize,
    height: usize,
    text: String,
    char_height: usize,
}

impl Canvas {
    pub fn new<S>(h: usize, text: S) -> Self
    where
        S: ToString,
    {
        let mut new = Self::default();
        new.height = h;
        new.text = text.to_string();
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
        self.width = (self.pad * 2) + (self.char_height * self.text.len());
    }

    pub fn generate_image(&self) -> GrayImage {
        let w = self.width as u32;
        let h = self.height as u32;
        let mut canv = DynamicImage::new_luma8(w, h);
        canv.invert();
        canv.to_luma()
    }

    fn overlay_text(&mut self) {
        self.text
            .chars()
            .enumerate()
            .for_each(|(i, v)| println!(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions() {
        let c = Canvas::new(100, 100);
        assert!(true);
    }
}
