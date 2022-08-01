use crate::misc::*;

fn sort_fun(m: &Pixels, j: usize) -> (i32, i32)
{

    let x1 = m.x[j+1];
    let y1 = m.y[j+1];

    let x0 = m.x[j];
    let y0 = m.y[j];

    if x0 > x1
    {
        return (x1, y1);
    }

    else {
        return (x0, y0);
    }

}

pub fn bubble_sort_algo(m: &mut Pixels)
{

    let mut m_tmp = Pixels { x: Vec::new(), y: Vec::new() };

    for j in (0..m.x.len()).step_by(2) {
        

        let (x, y) = sort_fun(m, j);
        

        m_tmp.x.push(x);
        m_tmp.y.push(y);

    }


    for i in 0..m_tmp.x.len() {
        
        m.x[i] = m_tmp.x[i];
        m.y[i] = m_tmp.y[i];

    }


}