use pixels::Pixels;

use crate::misc::*;
use crate::bubble_sort_algo::*;

fn binary_search(y0: i32, m: &mut PixelsCoordinate) -> i32
{

    let mut r = m.y.len() as i32;
    let mut l = 0;


    while l < r {

        let n = (l + r) / 2;
        
        if m.y[n as usize] < y0
        {
            l = n + 1;
        }        
        else
        {
            r = n;    
        }
    }


    return l;
}

pub fn scanline_algo(m: &mut PixelsCoordinate, canvas: &mut Pixels)
{
    bubble_sort_algo(m);

    for j in 0..(m.y.len() - 1) {
        
        let x0 = m.x[j];
        let y0 = m.y[j];



        let n = binary_search(y0, m);

        if n == -1
        {
            println!("error not found {j}");
            return;
        }
        let x1 = m.x[n as usize];

        
        if x0 < x1
        {
                draw_line(x0, x1, y0, [255, 0, 0], canvas); 
        }  
        else
        {
                draw_line(x1, x0, y0, [255, 0, 0], canvas); 
        }


    }

}