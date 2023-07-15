use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::controls::EventControls;
use crate::lookat_camera::Camera;
use crate::transformations::{scale, transform};
use crate::gltf_file_loader::GLTFLoader;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;


#[tokio::main]
pub async fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, sdl_context: &mut sdl2::Sdl)
{



    let mut i = 0.;
    let mut player_event = EventControls::init(0.0, 0.0, 10.0);

    let objet_data = GLTFLoader::load("./assets/personnage.glb");

    loop 
    {
    
        // encapsulation
        let mut objects = Vec::new();
        objects.push( objet_data.clone() );



        canvas.set_draw_color(sdl2::pixels::Color::RGB(155, 155, 155));
        canvas.clear(); 






        // transformations

        //objects[0].0 = rotate(&objects[0].0, PI, 'x');
        objects[0].0 = scale(&objects[0].0, 30.);
        //objects[0].0 = rotate(&objects[0].0, i as f32 * PI / 180., 'y');
        //objects[0].0 = translate(&objects[0].0, [0., -300., 0.]);
        //objects[0].0 = rotate(&objects[0].0, -i * PI / 180.0, 'z');


        // lecture des events
        if player_event.controls(event_pump, sdl_context)
        {
            break;
        };
        


        // TODO comprendre la rotation de la caméra

        // TODO A REVOIR

        let angle = i as f32 * PI / 180.;

        let rot_matrix = [
            [angle.cos(), 0., angle.sin(), 0.],
            [0., 1., 0. , 0.],
            [-angle.sin(), 0., angle.cos(), 0.],
            [0., 0., 0., 1.]
        ];
        
        let cam_pos = [angle.cos(), 0., angle.sin()];

        objects[0].0 = transform(&objects[0].0, rot_matrix);

        let cam_matrix = Camera::place(
            cam_pos, // from
            [0., 0., 0.], // to
            [0., 1., 0.]  // random_up
        );
        
        // ***************************

        // projection
        projection(&mut objects[0], cam_matrix);

        // colorisation
        Rasterizer::draw(&objects[0].0, &objects[0].1, &objects[0].2, canvas, &player_event).await;


      
   



        // angle de rotation
        i += 3.;
        i %= 360.0;
        

        // affichage
        canvas.present();
        


        // attente avant la prochaine itération
        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    
    }
}



