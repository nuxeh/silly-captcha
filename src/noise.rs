extern crate noise;

use noise::Seedable;
use noise::{Blend, Fbm, Perlin, RidgedMulti};
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use image::{ImageBuffer, GrayImage, FilterType, DynamicImage};
use rand::random;

pub struct Noise {
    width: usize,
    height: usize,
    map: NoiseMap,
    x: usize,
    y: usize,
}

impl Noise {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            x: 0,
            y: 0,
            map: NoiseMap::new(w, h),
        }
    }

    pub fn generate(&mut self) {
        let perlin = Perlin::new()
            .set_seed(random());
        let ridged = RidgedMulti::new()
            .set_seed(100);
        let fbm = Fbm::new()
            .set_seed(100);
        let blend = Blend::new(&perlin, &ridged, &fbm);

        self.map = PlaneMapBuilder::new(&blend)
            .set_size(self.width, self.height)
            .build();
    }
}

/// pixel iterator not present in noise crate
impl Iterator for Noise {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        self.x += 1;

        let val = self.map.get_value(self.x, self.y);
        if self.y < self.height && self.x < self.width {
            Some((clamp(val * 0.5 + 0.5, 0.0, 1.0) * 255.0) as u8)
        } else {
            None
        }
    }
}

/// non-accessible private math function used in noise crate
#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    assert!(max >= min);
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut n = Noise::new(10, 10);
        n.generate();
        for i in n {
            println!("{}", i);
        }
        assert!(false);
    }
}
