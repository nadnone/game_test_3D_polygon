use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::controls::EventControls;
use crate::transformations::{scale};
use crate::gltf_file_loader::GLTFLoader;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;


#[tokio::main]
pub async fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, sdl_context: &mut sdl2::Sdl)
{

    let mut i = 0.0;
    let mut player_event = EventControls::init(0.0, 0.0, 10.0);

    let mut objet_data = GLTFLoader::load("./assets/personnage.glb");
    
    //objet_data.0 = scale(&objet_data.0, 100.0);
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
        //objects[0].0 = rotate(&objects[0].0, i * PI / 180., 'y');
        //objects[0].0 = rotate(&objects[0].0, -i * PI / 180.0, 'z');


        // lecture des events
        if player_event.controls(event_pump, sdl_context)
        {
            break;
        };
        

        /* 
         Effectuer une camera FPS

         - https://youtu.be/o3yb7X_J9mw
         - https://gamedev.stackexchange.com/questions/189013/how-does-glmlookat-produce-a-view-matrix
         - https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/gluLookAt.xml
         - https://gamedev.stackexchange.com/questions/168542/camera-view-matrix-from-position-yaw-pitch-worldup
         - https://www.gamedev.net/forums/topic/638006-first-person-camera-and-matrix-transforms/5026831/
         - https://youtu.be/qByYk6JggQU

         yaw = rotation Z
            roll = rotation y
            pitch = rotation x

        */

        let cam_pos = player_event.get_pos_camera();
        let cam_angle = player_event.get_angle_camera();


        // TODO revoir le calcul des vecteurs d'axes

        let vec_x = [
            cam_angle[0].cos() * cam_angle[2].cos(),
            cam_angle[0].cos() * cam_angle[2].sin(),
            cam_angle[0].sin()
        ];

        let vec_y = [
            -cam_angle[2].sin(),
            cam_angle[2].cos(),
            0.
        ];

        let vec_z = [
            -cam_angle[0].sin() * cam_angle[2].cos(),
            -cam_angle[0].sin() * cam_angle[2].sin(),
            cam_angle[0].cos()

        ];


        // TODO comprendre le le fonctionnement de la caméra et son calcul

        let rotation_matrix = [
            [vec_x[0], vec_y[0], vec_z[0], -cam_pos[0]],
            [vec_x[1], vec_y[1], vec_z[1], -cam_pos[1]],
            [vec_x[2], vec_y[2], vec_z[2], -cam_pos[2]],
            [0., 0., 0., 1.]
        ];



        // **********************

        
        
        // projection
        projection(&mut objects[0], &player_event, rotation_matrix);

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



