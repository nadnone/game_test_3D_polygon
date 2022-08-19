use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::maths_vectors_helper::scalair;
use crate::transformations::{rotate, translate};
use crate::wavefront_parser;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{


    let (cube0, data_cube0) = wavefront_parser::load("./assets/cube.obj");
    let (cube1, data_cube1) = wavefront_parser::load("./assets/cube.obj");


    let mut i = 0.0;


    loop 
    {
        let mut cube_0: (Vec<[f32; 3]>, Vec<[f32; 3]>) = (cube0.clone(), cube0.clone());
        let mut cube_1: (Vec<[f32; 3]>, Vec<[f32; 3]>) = (cube1.clone(), cube1.clone());


        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 




        // lecture des events
        event_pump.poll_event();



        // transformations

        for i in 0..cube_0.1.len() {

            cube_0.1[i] = scalair(cube_0.1[i], 150.0);
            cube_1.1[i] = scalair(cube_1.1[i], 150.0);
            
        }


        cube_0.0 = rotate(&cube_0.1, i * 3.1415 / 180.0, 'y');
        cube_0.0 = rotate(&cube_0.0, i * 3.1415 / 180.0, 'z');
        cube_0.0 = translate(&cube_0.0, [400., 0., 0.]);


        cube_1.0 = rotate(&cube_1.1, i * 3.1415 / 180.0, 'y');
        cube_1.0 = rotate(&cube_1.0, i * 3.1415 / 180.0, 'z');
        cube_1.0 = translate(&cube_1.0, [-400., 0., 0.]);




        // colorisations
        let mut cubes = Vec::new();
        cubes.push( Rasterizer::draw(&cube_0.0, &data_cube0) );
        cubes.push( Rasterizer::draw(&cube_1.0, &data_cube1) );

    

        // projection
        projection(cubes, canvas);




        // angle de rotation
        i += 2.0;
        i %= 360.0;


        // affichage
        canvas.present();
        


        // attente avant la prochaine itération
        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    
    }
}



