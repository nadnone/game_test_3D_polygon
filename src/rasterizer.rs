use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{constants::{WIDTH_LOGIC, HEIGHT_LOGIC}, pseudo_shader, lookat_camera::Camera, maths_vectors_helper::{soustr_vec, produit_vectoriel, normaliser, produit_scalair}};
pub struct Rasterizer;


impl Rasterizer {

    pub async fn draw(data: &Vec<[f32; 4]>, normals: &Vec<[f32; 3]>, colors_data: &Vec<[f32; 3]>, canvas: &mut Canvas<Window>, camera_manager: &Camera)
    {

        for i in (0..data.len()).step_by(3) {


            if i+2 >= data.len()
            {
                println!("[!] Objet non triangulé");
                return;
            }

            let v0 = data[i + 0];
            let v1 = data[i + 1];
            let v2 = data[i + 2];


            // delete backfaces 

            // Calculate the triangle area
            let area = (v1[0] - v0[0]) * (v2[1] - v0[1]) - (v2[0] - v0[0]) * (v1[1] - v0[1]);

            // Check if the triangle is front-facing
            if area < 0.0 {
                continue; // Skip rendering the triangle if it's facing towards the camera
            }


            // to check less pixels at time
            let (min_x, max_x, min_y, max_y) = Self::_check_min_max(v0, v1, v2);


            // check overflow
            if min_x > i32::MAX - 1 ||
                min_y > i32::MAX - 1 ||
                max_x > i32::MAX - 1 ||
                max_y > i32::MAX - 1
            {
                return;
            }


            for px in min_x..max_x+1 {
                
                for py in min_y..max_y+1 {


                    // pixel coordinate
                    let p = [px as f32, py as f32, v0[2]];


                    // check if pixel is inside triangle
                    if Self::_is_in_triangle(p, v0, v1, v2) 
                    {

                        let rgb = pseudo_shader::shader(&camera_manager, &normals, p, &colors_data, i);

                        canvas.set_draw_color(sdl2::pixels::Color::RGB(rgb[0], rgb[1], rgb[2]));
                        canvas.draw_point(sdl2::rect::Point::new((px as f32 + WIDTH_LOGIC as f32 / 2.) as i32, (py as f32 + HEIGHT_LOGIC as f32 / 2.) as i32)).unwrap();
                    }
      
                }

            }


        }

    }

    fn _is_in_triangle(p: [f32; 3], a: [f32; 4], b: [f32; 4], c: [f32; 4]) -> bool
    {
        // positifs
        let mut check_pos = true;
        check_pos &= Self::_edge_check(p, c, a) > 0.0;
        check_pos &= Self::_edge_check(p, a, b) > 0.0;
        check_pos &= Self::_edge_check(p, b, c) > 0.0;

        // negatifs
        let mut check_neg = true;
        check_neg &= Self::_edge_check(p, c, a) < 0.0;
        check_neg &= Self::_edge_check(p, a, b) < 0.0;
        check_neg &= Self::_edge_check(p, a, c) < 0.0;

        
        return check_pos | check_neg; 

    }

    fn _edge_check(p: [f32; 3], a: [f32; 4], b: [f32; 4]) -> f32
    {
        // calcul du determinant
        return (a[0] - p[0]) * (b[1] - p[1]) - (a[1] - p[1]) * (b[0] - p[0]);
    }

    fn _check_min_max(v0: [f32; 4], v1: [f32; 4], v2: [f32; 4]) -> (i32, i32, i32, i32)
    {
        let max_x = *vec![v0[0] as i32, v1[0] as i32, v2[0] as i32].iter().max().unwrap();
        let min_x = *vec![v0[0] as i32, v1[0] as i32, v2[0] as i32].iter().min().unwrap();

        let max_y = *vec![v0[1] as i32, v1[1] as i32, v2[1] as i32].iter().max().unwrap();
        let min_y = *vec![v0[1] as i32, v1[1] as i32, v2[1] as i32].iter().min().unwrap();


        return (min_x, max_x, min_y, max_y)
    }

   
}
