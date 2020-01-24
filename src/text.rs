use font8x8::unicode::{BasicFonts, UnicodeFonts};
use image::GrayImage;

pub fn get_char_image(c: char) {
    let font = BasicFonts::new();
    let bytes = font.get(c);
    println!("{:?}", bytes);
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
        get_char_image('C');
        panic!("oh noes");
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
