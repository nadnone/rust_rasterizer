use std::fs::{self};
use std::io::Write;


pub struct Pixels {
    pub x: Vec<i32>,
    pub y: Vec<i32>
}

pub struct Item {
    pub x: f32,
    pub y: f32
}
pub struct ObjectDraw {
    pub obj: Vec<Item>
}




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

pub fn draw_pixel(x: i32, y: i32, c: [u8; 3])
{

}

/*pub fn draw(x: i32, y: i32, c: [u8; 3])
{

    let canvas = Canvas::new(800, 800)
                .title("Hello World")
                .render_on_change(true)
                .show_ms(true);



    canvas.render(move |mouse, image| {
       
        let width = image.width() as usize;
        let img = image;
        
        
        
        let scale = 10;

        for j in y..(y + scale) {
          
            for i in x..(x + scale) {

                drawPixel(img, width, i, j, c)
            
            }

        }




    });

    



}*/