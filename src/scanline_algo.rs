use crate::misc::*;
use crate::bubble_sort_algo::*;

pub fn scanline_algo(m: &mut Pixels)
{
    bubble_sort_algo(m);

    for i in 0..(m.x.len() - 1) {
        

        let x0 = m.x[i];
        let y0 = m.y[i];


    

        let mut x_list: Vec<i32> = Vec::new();



        for k in i..m.x.len() {
            
            if y0 == m.y[k]
            {
                x_list.push(m.x[k]);
            }
        }


        let x1 = x_list[x_list.len()-1];


        if x0 < x1
        {
            for x in x0..x1 {
                
                //draw(x, y, [200, 100, 0]);
                draw_pixel(x, y0, [200, 100, 0]); 
                
            }
        }
        else
        {
            for x in x1..x0 {
                
                //draw(x, y, [200, 100, 0]);
                draw_pixel(x, y0, [200, 100, 0]); 


            }
        }


    }

}