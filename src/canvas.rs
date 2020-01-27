use image::{Pixel, Luma, ImageBuffer, GrayImage, FilterType, DynamicImage, imageops::overlay};
use std::convert::TryInto;

use crate::character::Character;
use crate::noise::Noise;

#[derive(Clone)]
pub struct Canvas {
    text: String,
    pad: usize,
    height: usize,
    native_size: (usize, usize),
    width: Option<usize>,
    blur: Option<f32>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            pad: 3,
            height: 100,
            native_size: (0, 0),
            width: None,
            blur: Some(0.0),
            blend_type: BlendType::default(),
        }
    }
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

    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
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
        let w = (self.text.len() * 8) + (2 * self.pad);
        let h = 7 + (2 * self.pad);
        self.native_size = (w, h);
        self.clone()
    }

    pub fn get_width(&self) -> usize {
        let (w, h) = self.native_size;

        self.width.unwrap_or_else(|| {
            (w / h) * self.height
        })
    }

    pub fn generate_image(&self) -> GrayImage {
        let (w, h) = self.native_size;

        let w = w.try_into().unwrap();
        let h = h.try_into().unwrap();

        // make a white coloured base image of the correct dimensions
        let mut canv = DynamicImage::new_luma8(w, h);
        canv.invert();

        self.overlay_text(&mut canv);

        let w = self.get_width().try_into().unwrap();
        let h = self.height.try_into().unwrap();

        let canv = canv.resize_exact(w, h, FilterType::Nearest);

        let canv = if let Some(sigma) = self.blur {
            canv.blur(sigma)
        } else {
            canv
        };

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
        let canvas = Canvas::new(100, "ads123dahj31kjdhagq")
            .pad(3)
//            .blur(2.0)
            .build();

        let mut n = Noise::new(canvas.get_width(), canvas.height);
        n.generate();

        let mut blended = canvas.generate_image();

        blended
            .pixels_mut()
            .zip(n)
            .for_each(|(mut a, b)| {
                let Luma(cur) = a;
                //let blended = cur[0] ^ b;
                let blended: i16 = cur[0] as i16 - b as i16;
                let blended = blended.abs() as u8;
                *a = Luma([blended]);
            });

        blended.save("/tmp/bl.png");
        assert!(false);
    }

    #[test]
    fn test_dimensions() {
        let c = Canvas::new(100, "ads123dahj31kjdhagq")
            .pad(2)
            .blur(0.1)
            //.blur(5.0)
            .build();
        let img = c.generate_image();
        img.save("/tmp/cheese.pgm");
        assert_eq!(img.height(), 1000);
    }
}
