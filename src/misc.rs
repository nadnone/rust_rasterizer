use pixels::Pixels;

pub struct PixelsCoordinate {
    pub x: Vec<i32>,
    pub y: Vec<i32>
}

pub struct Item {
    pub x: i32,
    pub y: i32
}
pub struct ObjectDraw {
    pub obj: Vec<Item>
}

pub const WIDTH:u32 = 100;
pub const HEIGHT:u32 = 100;
pub const FPS:f32 = 1.0/60.0;
pub const FACTOR_SIZE:f64 = 8.0;


/*
pub fn log(message: String)
{
    let mut file = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .open("log.txt")
        .unwrap();

        
        if write!(file, "{message}").is_err()
        {
            println!("can't log");
        }

    
}
*/

fn ix(x: i32, y: i32) -> usize
{
    return ( x + y * WIDTH as i32) as usize;
}

pub fn draw_line(x0: i32, x1: i32, y: i32, c: [u8; 3], canvas: &mut Pixels)
{
    for x in x0..x1 {
        draw_pixel(x, y, c, canvas);
    }
}

fn draw_pixel(x: i32, y: i32, c: [u8; 3], canvas: &mut Pixels)
{
    
    let frame = canvas.get_frame();
    
    let color = [c[0], c[1], c[2], 255];


  
    for pixel in frame.chunks_exact_mut(4).skip(ix(x, y)) {
       
        pixel.copy_from_slice(&color);
        break;
    }

   
}
