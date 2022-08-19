use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::controls::EventControls;
use crate::maths_vectors_helper::scalair;
use crate::transformations::{rotate, translate};
use crate::wavefront_parser;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{


    let cube1 = wavefront_parser::load("./assets/plane_animation.obj");
    let cube2 = wavefront_parser::load("./assets/cube.obj");


    let mut i = 0.0;
    let mut player_event = EventControls::init(0.0, 0.0, 0.0);


    loop 
    {
    
        let mut projection_data = Vec::new();


        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 




        // encapsulation
        let mut objects = Vec::new();
        objects.push( cube1.clone() );
        objects.push( cube2.clone() );


        // transformations
        for j  in 0..objects.len() {
            

            for i in 0..objects[j].0.len() {

                objects[j].0[i] = scalair(objects[j].0[i], 150.0);
                
            }
    
            objects[j].0 = rotate(&objects[j].0, i * PI / 180.0, 'x');
            objects[j].0 = rotate(&objects[j].0, i * PI / 180.0, 'z');
            objects[j].0 = translate(&objects[j].0, [-400., 0., 0.]);
    
            
    
    
            // lecture des events
            player_event.controls(&mut objects, event_pump);
            



            projection_data.push(  Rasterizer::draw(&objects[j]) );


        }
       


        // projection
        projection(projection_data, canvas);




        // angle de rotation
        i += 2.0;
        i %= 360.0;
        


        // affichage
        canvas.present();
        


        // attente avant la prochaine itération
        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    
    }
}



