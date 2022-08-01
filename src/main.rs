mod misc;
mod bresenham_algo;
mod scanline_algo;
mod bubble_sort_algo;

use crate::misc::*;



use winsafe::prelude::*;
use winsafe::{gui, POINT, SIZE};


fn rasterizer()
{
    let item1 = Item { x: 100.0, y: 600.0 };
    let item2 = Item { x: 600.0, y: 600.0 };
    let item3 = Item { x: 600.0, y: 100.0 };
    let item4 = Item { x: 100.0, y: 600.0 };

    let mut objet = ObjectDraw { obj: vec![ item1, item2, item3, item4 ] };
    let mut m = misc::Pixels { x: Vec::new(), y: Vec::new() };

    bresenham_algo::bresenham_algo(&mut m, &mut objet);



    scanline_algo::scanline_algo(&mut m);
    
      
}


fn main()
{

    // trouver un moyen de faire un GUI

}