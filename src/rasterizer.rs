use crate::misc::{PixelsCoordinate, ObjectDraw};
use crate::bresenham_algo::bresenham_algo;
use crate::scanline_algo::scanline_algo;

use pixels::Pixels;

pub struct Rasterizer {
    pub m: PixelsCoordinate,
}

impl Rasterizer {

    pub fn init(m: &mut PixelsCoordinate, objet_in: &mut ObjectDraw)
    {
        bresenham_algo(m, objet_in);

    }
    pub fn draw(m: &mut PixelsCoordinate, canvas: &mut Pixels)
    {
        scanline_algo(m, canvas);
    }
}

