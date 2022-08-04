use crate::misc::*;

fn sort_fun(m: &PixelsCoordinate, j: usize) -> (i32, i32)
{

    let x1 = m.x[j+1];
    let y1 = m.y[j+1];

    let x0 = m.x[j];
    let y0 = m.y[j];

    if x0 > x1
    {
        return (x1, y1);
    }
    else 
    {
        return (x0, y0);
    }

}

pub fn bubble_sort_algo(m: &mut PixelsCoordinate)
{

    for j in 0..(m.y.len() -1) {
        
        let (x, y) = sort_fun(m, j);
        
        m.x[j] = x;
        m.y[j] = y;

    }

}