use font8x8::unicode::{BasicFonts, UnicodeFonts};
use image::{GrayImage, FilterType, DynamicImage};
use std::error::Error;

pub struct Character {
    image: GrayImage,
}

impl Character {
    pub fn new(c: char) -> Result<Self, Box<dyn Error>> {
        let mut raw_image = vec![];

        let bytes = if let Some(bytes) = get_char_bytes(c) {
            bytes
        } else {
            return Err(format!("invalid character '{}' requested", c).into());
        };

        bytes
            .iter()
            .for_each(|b| raw_image.extend(byte_to_vec(*b)));

        let image = GrayImage::from_raw(8, 8, raw_image).unwrap();

        Ok(Self {image})
    }

    pub fn get_image_buffer(&self) -> &GrayImage {
        &self.image
    }

// move occurs because `self.image` has type `image::ImageBuffer<image::Luma<u8>,
// std::vec::Vec<u8>>`, hhich does not implement the `Copy` trait
//    pub fn as_image(&self) -> DynamicImage {
//        DynamicImage::ImageLuma8(self.image)
//    }

    pub fn get_image(&self) -> DynamicImage {
        DynamicImage::ImageLuma8(self.image.clone())
    }

    pub fn generate_image(&self, height: usize) -> DynamicImage {
        let h = height as u32;
        let d = DynamicImage::ImageLuma8(self.image.clone())
            .resize_exact(h, h, FilterType::Nearest);
        d
    }
}

fn get_char_bytes(c: char) -> Option<[u8; 8]> {
    let font = BasicFonts::new();
    font.get(c)
}

fn byte_to_vec(b: u8) -> Vec<u8> {
    let mut res = vec![];
    for i in 0..8 {
        if b & (1 << i) > 0 {
            res.push(0);
        } else {
            res.push(255);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char_image() {
        let mut c = Character::new('9').unwrap();
        let i = c.get_image_buffer();
        let d = DynamicImage::ImageLuma8(i.clone());
    }

    #[test]
    fn test_get_char_bytes() {
        assert_eq!(get_char_bytes('C'), Some([60, 102, 3, 3, 3, 102, 60, 0]));
    }

    #[test]
    fn test_byte_to_vec() {
        assert_eq!(byte_to_vec(60),  [255, 255, 0,   0,   0,   0,   255, 255]);
        assert_eq!(byte_to_vec(102), [255, 0,   0,   255, 255, 0,   0,   255]);
        assert_eq!(byte_to_vec(3),   [0,   0,   255, 255, 255, 255, 255, 255]);
        assert_eq!(byte_to_vec(3),   [0,   0,   255, 255, 255, 255, 255, 255]);
        assert_eq!(byte_to_vec(3),   [0,   0,   255, 255, 255, 255, 255, 255]);
        assert_eq!(byte_to_vec(102), [255, 0,   0,   255, 255, 0,   0,   255]);
        assert_eq!(byte_to_vec(60),  [255, 255, 0,   0,   0,   0,   255, 255]);
        assert_eq!(byte_to_vec(0),   [255, 255, 255, 255, 255, 255, 255, 255]);
    }
}
