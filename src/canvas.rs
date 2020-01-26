use image::{ImageBuffer, GrayImage, FilterType, DynamicImage, imageops::overlay};

use crate::character::Character;

#[derive(Clone, Default)]
pub struct Canvas {
    pad: usize,
    width: usize,
    height: usize,
    text: String,
    char_height: usize,
    px_height: usize,
    blur: Option<f32>,
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

    pub fn blur(&mut self, sigma: f32) -> &mut Self {
        self.blur = Some(sigma);
        self
    }

    pub fn text<S>(&mut self, text: S) -> &mut Self
    where
        S: ToString,
    {
        self.text = text.to_string();
        self
    }

    pub fn build(&mut self) -> Self {
        self.px_height = self.height / (7 + (2 * self.pad));
        self.char_height = self.px_height * 8;
        self.width = (self.pad * 2) + (self.char_height * self.text.len());
        self.clone()
    }

    pub fn generate_image(&self) -> GrayImage {
        let w = self.width as u32;
        let h = self.height as u32;

        // make a white coloured base image of the correct dimensions
        let mut canv = DynamicImage::new_luma8(w, h);
        canv.invert();

        self.overlay_text(&mut canv);

        let canv = if let Some(sigma) = self.blur {
            canv.blur(sigma)
        } else {
            canv
        };

        canv.to_luma()
    }

    fn overlay_text(&self, image: &mut DynamicImage) {
        let start = self.pad * self.px_height;
        self.text
            .chars()
            .enumerate()
            .for_each(|(i, v)| {
                let x = start + (self.char_height * i);
                self.overlay_character(image, v, x, self.pad)
            });
    }

    fn overlay_character(&self, image: &mut DynamicImage, c: char, x: usize, y: usize) {
        let glyph = Character::new(c).unwrap().generate_image(self.char_height);
        if c == 'l' {
            glyph.save("/tmp/glyph.png");
        }
        overlay(image, &glyph, x as u32, y as u32);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions() {
        let c = Canvas::new(100, "ads123dahj31kjdhagq")
            .pad(20)
            .blur(5.0)
            .build();
        c.generate_image().save("/tmp/cheese.pgm");
        //assert!(false);
    }
}
