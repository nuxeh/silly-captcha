use font8x8::unicode::{BasicFonts, UnicodeFonts};
use image::GrayImage;

pub fn get_char_image(c: char) {
    let font = BasicFonts::new();
    let bytes = font.get(c);
    println!("{:?}", bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char_image() {
        get_char_image('C');
        panic!("oh noes");
    }
}
