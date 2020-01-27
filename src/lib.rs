extern crate font8x8;
extern crate image;

pub mod canvas;
pub mod character;
pub mod noise;

/// Struct used to configure and build a Captcha
#[derive(Clone)]
struct Captcha {
    width: usize,
    height: usize,
    padding: usize,
    num_chars: usize,
    text: Option<String>,
    data: Vec<u8>,
}

impl Default for Captcha {
    fn default() -> Self {
        Self {
            width: 100,
            height: 50,
            padding: 5,
            num_chars: 4,
            text: None,
            data: vec![],
        }
    }
}

impl Captcha {
    /// Obtain an uninitialized Captcha
    fn new() -> Self {
        Self::default()
    }

    /// Set the width of the generated captcha image.
    fn width(&mut self, width: usize) -> &mut Self {
        self.width =  width;
        self
    }

    /// Set the height of the generated captcha image.
    fn height(&mut self, height: usize) -> &mut Self {
        self.height =  height;
        self
    }

    /// Set the padding of the generated captcha image.
    fn padding(&mut self, padding: usize) -> &mut Self {
        self.padding =  padding;
        self
    }

    /// Set the num_chars of the generated captcha image.
    fn num_chars(&mut self, num_chars: usize) -> &mut Self {
        self.num_chars =  num_chars;
        self
    }

    /// Generate a random string of specified length.
    fn random(&mut self, length: u8) {

    }

    /// Use random dictionary entries
    ///
    /// Requires Unix, and a dictionary file to be available on the system.
    fn dict(&mut self, num_words: u8) {

    }

    /// Set the text to display (don't auto-generate)
    fn text<S>(&mut self, text: &S) -> &mut Self
    where
        S: ToString,
    {
        self.text = Some(text.to_string());
        self
    }

    /// Initialise the Captcha struct
    fn build(&mut self) -> Self {
        self.clone()
    }

    fn as_byte_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

//    fn as_generic_image(&self) -> GenericImage {
//
//    }
}
