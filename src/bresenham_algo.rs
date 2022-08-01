use crate::misc::*;



pub fn bresenham_algo(m: &mut Pixels, objet: &mut ObjectDraw)
{

        for k in 0..(objet.obj.len() -1)  {
            
            condition_loop(m, objet, k, 1);
        }
        condition_loop(m, objet, 0, objet.obj.len() - 1);


}
fn calculus_y(objet: &mut ObjectDraw, k: usize, m: &mut Pixels, l: usize)
{
    let a;
    let b;

    if objet.obj[k].x < objet.obj[k+l].x
    {
        a = 0; b = 1;
    }
    else {
        a = 1; b = 0;
    }


    if objet.obj[k+a].y == objet.obj[k+b].y
    {
        let y = objet.obj[k+b].y;

        let mut counter_x = objet.obj[k+a].x;

        while counter_x < objet.obj[k+b].x {
            
            m.x.push(counter_x as i32);
            m.y.push(y as i32);

            counter_x += 1.0;

        }

    }

}
fn calculus_x(objet: &mut ObjectDraw, k: usize, m: &mut Pixels, l: usize)
{

    let a;
    let b;

    if objet.obj[k].y < objet.obj[k+l].y
    {
            a = 0; b = 1;
    }
    else {
        a = 1; b = 0;
    }


    if objet.obj[k+a].x == objet.obj[k+b].x
    {
        let x = objet.obj[k+b].x;

        let mut counter_y = objet.obj[k+a].y;

        while counter_y < objet.obj[k+b].y {
            
            m.x.push(x as i32);
            m.y.push(counter_y as i32);

            counter_y += 1.0;

        }

    }

}
fn bresenham_calculus(objet: &mut ObjectDraw, k: usize, m: &mut Pixels, l0: usize)
{

    let mut a = 0; 
    let mut l = l0;

    if objet.obj[k].x > objet.obj[k+1].x
    {
         a = l0; l = 0;
    }


    let mut counter: f32 = objet.obj[k+a].x;
    while counter < objet.obj[k+l].x {
        
        if objet.obj[k+a].x >= objet.obj[k+l].x
        {
            break;
        }
        counter += 1.0;


        let x0 = objet.obj[k+a].x;  let x1 = objet.obj[k+l].x;
        let y0 = objet.obj[k+a].y;  let y1 = objet.obj[k+l].y; 
        
        
        let y = ( ( y1 - y0 ) / ( x1 - x0 ) * ( counter - x0 ) ) + y0;

        m.y.push(y as i32);
        m.x.push(counter as i32);

    }

}

fn condition_loop(m: &mut Pixels, objet: &mut ObjectDraw, k: usize, l: usize)
{

    calculus_y(objet, k, m, l);
    calculus_x(objet, k, m, l);
    bresenham_calculus(objet, k, m, l);
}



