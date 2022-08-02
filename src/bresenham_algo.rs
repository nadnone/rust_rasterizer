use crate::misc::*;

pub fn bresenham_algo(m: &mut PixelsCoordinate, objet: &mut ObjectDraw)
{
    for k in 0..(objet.obj.len() -1)  {
        
        condition_loop(m, objet, k, 1);

    }
    condition_loop(m, objet, 0, objet.obj.len() - 1);

}


fn bresenham_calculus(objet: &mut ObjectDraw, k: usize, m: &mut PixelsCoordinate, l0: usize)
{

    let mut a = 0; 
    let mut l = l0;

    if objet.obj[k].x > objet.obj[k+l].x
    {
         a = l;
         l = l0;
    }
    
    let mut x0 = objet.obj[k+a].x as f32;  let mut x1 = objet.obj[k+l].x as f32;
    let mut y0 = objet.obj[k+a].y as f32;  let mut y1 = objet.obj[k+l].y as f32;


    for x in objet.obj[k+a].x .. (objet.obj[k+l].x + 1)
    {   
        let y = ( ( y1 - y0 ) / ( x1 - x0 ) * ( x as f32 - x0 ) ) + y0;

        m.x.push(x);
        m.y.push(y as i32);

    }


    if objet.obj[k].y > objet.obj[k+l].y
    {
            a = l;
            l = l0;
    }

    x0 = objet.obj[k+a].x as f32;  x1 = objet.obj[k+l].x as f32;
    y0 = objet.obj[k+a].y as f32;  y1 = objet.obj[k+l].y as f32;

    for y in objet.obj[k+a].y .. (objet.obj[k+l].y + 1)
    {   
        let x = ( ( x1 - x0 ) /  ( y1 - y0 ) * ( y as f32 - x0 ) ) + x0;

        m.x.push(x as i32);
        m.y.push(y);

    }

    

}

fn condition_loop(m: &mut PixelsCoordinate, objet: &mut ObjectDraw, k: usize, l: usize) 
{

    bresenham_calculus(objet, k, m, l);

}



