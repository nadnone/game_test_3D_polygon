use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::controls::EventControls;
use crate::transformations::{rotate, translate, scale};
use crate::gltf_file_loader::GLTFLoader;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;


#[tokio::main]
pub async fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{

    let mut i = 0.0;
    let mut player_event = EventControls::init(0.0, 0.0, 10.0);

    let mut objet_data = GLTFLoader::load("./assets/personnage.glb");
    
    objet_data.0 = scale(&objet_data.0, 60.0);
    //objet_data.0 = rotate(&objet_data.0, 180.0 * PI / 180.0, 'x');
    //objet_data.0 = translate(&objet_data.0, [0., HEIGHT/2.0, 0.]);
    
    loop 
    {
    
        // encapsulation
        let mut objects = Vec::new();
        objects.push( objet_data.clone() );



        canvas.set_draw_color(sdl2::pixels::Color::RGB(155, 155, 155));
        canvas.clear(); 






        // transformations

        //objects[0].0 = rotate(&objects[0].0, i * PI / 180.0, 'x');
        objects[0].0 = rotate(&objects[0].0, i * PI / 180.0, 'y');
        //objects[0].0 = rotate(&objects[0].0, -i * PI / 180.0, 'z');


        // lecture des events
        if player_event.controls(event_pump)
        {
            break;
        };
        

        // projection
        projection(&mut objects[0], &player_event);

        // colorisation
        Rasterizer::draw(&objects[0].0, &objects[0].1, &objects[0].2, canvas, &player_event).await;

       
   



        // angle de rotation
        i += 2.0;
        i %= 360.0;
        


        // affichage
        canvas.present();
        


        // attente avant la prochaine itération
        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    
    }
}



