use crate::constants::*;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub fn projection(m: Vec<(Vec<[f32; 3]>, Vec<[u8; 3]>)>, canvas: &mut Canvas<Window>)
{




    let z_far = 100.0;
    let z_near = 1.0;
    let f: f32 = (FOV/2.0).tan() / 2.0;
    let lambda: f32 = z_far / (z_far - z_near); // Z_FAR / (Z_FAR - Z_NEAR)


    for objet in m {


        for i in 0..objet.0.len() {
        

            let x0 = objet.0[i][0];
            let y0 = objet.0[i][1];
            let z0 = objet.0[i][2]; 
    
            let colors = objet.1[i];
    
    
            let x = (HEIGHT/WIDTH) * f * x0;
            let y = f * y0;
            let _z = lambda * (z0 - 1.0); // 1.0 = Z_NEAR
    
            
            
            canvas.set_draw_color(sdl2::pixels::Color::RGB(colors[0], colors[1], colors[2]));
            canvas.draw_point(sdl2::rect::Point::new(x as i32 + WIDTH_LOGIC /2, y as i32 + HEIGHT_LOGIC /2)).unwrap();
        }
    

    }

    
}