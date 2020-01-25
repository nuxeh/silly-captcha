use font8x8::unicode::{BasicFonts, UnicodeFonts};
use image::{ImageBuffer, GrayImage, FilterType, DynamicImage};
use std::error::Error;

pub struct Character {
    bytes: [u8; 8],
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

        Ok(Self {bytes, image})
    }

    pub fn get_image(&mut self) -> &GrayImage {
        &self.image
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
        let i = c.get_image();
        let d = DynamicImage::ImageLuma8(i.clone());
        d.resize(40, 40, FilterType::Nearest).blur(2.0).save("/tmp/sb.png");
        let mut blank = DynamicImage::new_luma8(240, 240);
        blank.invert();
        blank.save("/tmp/blank.png");
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
