use image::{ImageBuffer, GrayImage, FilterType, DynamicImage, imageops::overlay};
use std::convert::TryInto;

use crate::character::Character;
use crate::noise::Noise;

#[derive(Clone, Default)]
pub struct Canvas {
    pad: usize,
    width: usize,
    height: usize,
    native_width: usize,
    text: String,
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
        self.width = (self.text.len() * 8) + (2 * self.pad);
        self.height = 7 + (2 * self.pad);
        self.clone()
    }

    pub fn generate_image(&self) -> GrayImage {
        let w = self.width.try_into().unwrap();
        let h = self.height.try_into().unwrap();

        // make a white coloured base image of the correct dimensions
        let mut canv = DynamicImage::new_luma8(w, h);
        canv.invert();

        self.overlay_text(&mut canv);

        let canv = if let Some(sigma) = self.blur {
            canv.blur(sigma)
        } else {
            canv
        };

        let canv = canv.resize(1000 as u32, self.height as u32, FilterType::Nearest);
        canv.to_luma()
    }

    fn overlay_text(&self, image: &mut DynamicImage) {
        self.text
            .chars()
            .enumerate()
            .for_each(|(i, v)| {
                let x = self.pad + (8 * i);
                self.overlay_character(image, v, x, self.pad)
            });
    }

    fn overlay_character(&self, image: &mut DynamicImage, c: char, x: usize, y: usize) {
        let glyph = Character::new(c).unwrap().get_image();
        overlay(image, &glyph, x as u32, y as u32);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend() {
        let c = Canvas::new(100, "ads123dahj31kjdhagq")
            .pad(2)
            .build();

        let mut n = Noise::new(c.width, c.height);
        n.generate();

        c.generate_image()
            .pixels()
            .zip(n)
            .for_each(|(a, b)| println!("{:?} {:?}", a, b));

        assert!(false);
    }

    #[test]
    fn test_dimensions() {
        let c = Canvas::new(100, "ads123dahj31kjdhagq")
            .pad(2)
            .blur(0.1)
            //.blur(5.0)
            .build();
        c.generate_image().save("/tmp/cheese.pgm");
        //assert!(false);
    }
}
