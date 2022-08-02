use crate::misc::*;

pub fn bresenham_algo(m: &mut PixelsCoordinate, objet: &mut ObjectDraw)
{
    for k in 0..(objet.obj.len() -1)  {
        
        condition_loop(m, objet, k, 1);

    }
    condition_loop(m, objet, 0, objet.obj.len() - 1);

}
fn calculus_y(objet: &mut ObjectDraw, k: usize, m: &mut PixelsCoordinate, l: usize)
{
    let a;
    let b;

    if objet.obj[k].x < objet.obj[k+l].x
    {
        a = 0; b = l;
    }
    else {
        a = l; b = 0;
    }


    if objet.obj[k+a].y == objet.obj[k+b].y
    {
        let y = objet.obj[k].y;

        for x in objet.obj[k+a].x .. (objet.obj[k+b].x + 1) 
        {
            
            m.x.push(x);
            m.y.push(y);

        }

    }

}
fn calculus_x(objet: &mut ObjectDraw, k: usize, m: &mut PixelsCoordinate, l: usize)
{

    let a;
    let b;

    if objet.obj[k].y < objet.obj[k+l].y
    {
        a = 0; b = l;
    }
    else {
        a = l; b = 0;
    }


    if objet.obj[k+a].x == objet.obj[k+b].x
    {
        let x = objet.obj[k].x;


        for y in objet.obj[k+a].y .. (objet.obj[k+b].y + 1)
        {
            
            m.x.push(x);
            m.y.push(y);


        }

    }

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
    
        let x0 = objet.obj[k+a].x as f32;  let x1 = objet.obj[k+l].x as f32;
        let y0 = objet.obj[k+a].y as f32;  let y1 = objet.obj[k+l].y as f32;
    
    
        for x in objet.obj[k+a].x .. (objet.obj[k+l].x + 1)
        {   
            let y = ( ( y1 - y0 ) / ( x1 - x0 ) * ( x as f32 - x0 ) ) + y0;

            m.x.push(x);
            m.y.push(y as i32);

        }

    

}

fn condition_loop(m: &mut PixelsCoordinate, objet: &mut ObjectDraw, k: usize, l: usize) 
{
    calculus_y(objet, k, m, l);
    calculus_x(objet, k, m, l);
    bresenham_calculus(objet, k, m, l);

}



