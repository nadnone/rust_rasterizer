mod misc;
mod bresenham_algo;
mod scanline_algo;
mod bubble_sort_algo;
mod rasterizer;
mod game_loop;


use std::vec;

use crate::rasterizer::*;
use crate::game_loop::*;
use crate::misc::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{ 
    dpi::LogicalSize, 
    window::WindowBuilder,
    event_loop::{EventLoop}
};


fn main()
{
    let item1 = Item { x: 10, y: 10 };
    let item2 = Item { x: 10, y: 50 };
    let item3 = Item { x: 50, y: 50 };

    let item4 = Item { x: 10, y: 10 };


    let mut objet = ObjectDraw { obj: vec![ item1, item2, item3, item4 ] };
    let mut m = PixelsCoordinate { x: Vec::new(), y: Vec::new() };

    Rasterizer::init(&mut m, &mut objet);



    let event_loop = EventLoop::new();

    let window = {

        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * FACTOR_SIZE, HEIGHT as f64 * FACTOR_SIZE);

        WindowBuilder::new()
            .with_title("Rasterizer")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut canvas = Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(window.inner_size().width, window.inner_size().height, &window)).unwrap();


    game_loop(&mut m, &mut canvas);

}



