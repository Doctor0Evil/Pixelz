// crate: pixelz-core/src/lib.rs
use serde::{Deserialize, Serialize};

pub const PIXELZ_CANVAS_W: u32 = 64;
pub const PIXELZ_CANVAS_H: u32 = 64;
pub const PIXELZ_LAYERS:   u8  = 4;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelCanvas {
    pub width:  u32,
    pub height: u32,
    pub layers: u8,
    // Flattened: layer-major, then row-major
    pub pixels: Vec<Rgba>,
}

impl PixelCanvas {
    pub fn new() -> Self {
        let n = PIXELZ_CANVAS_W as usize
            * PIXELZ_CANVAS_H as usize
            * PIXELZ_LAYERS as usize;
        Self {
            width:  PIXELZ_CANVAS_W,
            height: PIXELZ_CANVAS_H,
            layers: PIXELZ_LAYERS,
            pixels: vec![Rgba { r: 0, g: 0, b: 0, a: 0 }; n],
        }
    }

    #[inline]
    fn idx(&self, layer: u8, x: u32, y: u32) -> Option<usize> {
        if layer as u32 >= self.layers as u32 || x >= self.width || y >= self.height {
            return None;
        }
        let li = layer as usize;
        let i = li * (self.width as usize * self.height as usize)
              + (y as usize * self.width as usize)
              + (x as usize);
        Some(i)
    }

    pub fn set_pixel(&mut self, layer: u8, x: u32, y: u32, c: Rgba) -> bool {
        if let Some(i) = self.idx(layer, x, y) {
            self.pixels[i] = c;
            true
        } else {
            false
        }
    }

    pub fn get_pixel(&self, layer: u8, x: u32, y: u32) -> Option<Rgba> {
        self.idx(layer, x, y).map(|i| self.pixels[i])
    }
}
