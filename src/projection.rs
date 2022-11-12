use crate::constants::*;
use crate::controls::EventControls;


/// ```
/// data: (points, (normals, phon_data)), 
pub fn projection(data: &mut (Vec<[f32; 6]>, Vec<[f32; 3]>, Vec<[f32; 3]>), player_event: &EventControls)
{





    // déplacement de la caméra
    let camera_position = player_event.get_pos_camera(); // coordonnées de la caméra [x,y,z]

    // A REVOIR
    let move_z = FOV/camera_position[2]; // angle de vue sur position caméra Z

    for i in 0..data.0.len() {

        let point = data.0[i]; // une coordonnnées d'un triangle [x,y,z]


        let x0 = (point[0] - camera_position[0]) * (move_z).tan();
        let y0 = (point[1] - camera_position[1]) * (move_z).tan();
        let z0 = point[2] - camera_position[2];

        data.0[i][0] = x0;
        data.0[i][1] = y0;
        data.0[i][2] = z0;

    }   

    
}