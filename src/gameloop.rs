use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::controls::EventControls;
use crate::transformations::{rotate, translate, scale, reset_translation};
use crate::wavefront_parser;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{


    let cube1 = wavefront_parser::load("./assets/plane_animation.obj");
    let cube2 = wavefront_parser::load("./assets/cube.obj");


    let mut i = 0.0;
    let mut player_event = EventControls::init(0.0, 0.0, 100.0);


    
    
    loop 
    {
    
        // encapsulation
        let mut objects = Vec::new();
        objects.push( cube1.clone() );
        objects.push( cube2.clone() );



        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 






        // transformations

        objects[0].0 = scale(&objects[0].0, 150.0);
        objects[1].0 = scale(&objects[1].0, 150.0);
      
        objects[0].0 = rotate(&objects[0].0, i * PI / 180.0, 'x');
        objects[0].0 = rotate(&objects[0].0, i * PI / 180.0, 'z');


        objects[1].0 = reset_translation(&objects[1].0);

        objects[1].0 = rotate(&objects[1].0, i * PI / 180.0, 'x');
        objects[1].0 = rotate(&objects[1].0, i * PI / 180.0, 'z');

        objects[1].0 = translate(&objects[1].0, [-400., 0., 0.]);
        


        // lecture des events
        if player_event.controls(event_pump)
        {
            break;
        };
        

        // projection
        projection(&mut objects[0], &player_event);
        projection(&mut objects[1], &player_event);

        // colorisation
        Rasterizer::draw(&objects[1].0, &objects[1].1.0, &objects[1].1.1, canvas, &player_event);
        Rasterizer::draw(&objects[0].0, &objects[0].1.0, &objects[0].1.1, canvas, &player_event);

   





        // angle de rotation
        i += 2.0;
        i %= 360.0;
        


        // affichage
        canvas.present();
        


        // attente avant la prochaine itération
        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    
    }
}



