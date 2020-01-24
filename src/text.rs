use font8x8::unicode::{BasicFonts, UnicodeFonts};
use image::{ImageBuffer,GrayImage};
use std::error::Error;

pub struct Character {
    bytes: [u8; 8],
    pixels: Vec<u8>,
}

impl Character {
    pub fn new(c: char) -> Result<Self, Box<dyn Error>> {

        if let Some(bytes) = get_char_bytes(c) {
            let res = Self {
                bytes,
                pixels: vec![],
            };
            Ok(res)
        } else {
            Err(format!("invalid character '{}' requested", c).into())
        }
    }

    pub fn get_image(&self) -> GrayImage {
        let mut res = vec![];

        self.bytes
            .iter()
            .for_each(|b| res.extend(byte_to_vec(*b)));

        println!("{:?}", res);
        GrayImage::from_raw(8, 8, res).unwrap()
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
        let c = Character::new('C');
        c.unwrap().get_image().save("/tmp/s.pgm");
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
