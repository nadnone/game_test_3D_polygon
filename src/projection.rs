use crate::constants::*;
use crate::controls::EventControls;


/// ```
/// data: (points, (normals, phon_data)), 
pub fn projection(data: &mut (Vec<[f32; 6]>, Vec<[f32; 3]>, Vec<[f32; 3]>), player_event: &EventControls)
{



    // pour éviter d'avoir des objets en négatifs
    let z_far = 1000.0;
    let z_near = 1.0;
    let lambda: f32 = z_far / (z_far - z_near); // Z_FAR / (Z_FAR - Z_NEAR)

    // déplacement de la caméra
    let camera_position = player_event.get_pos_camera();
    // A REVOIR
    let move_z = (2.0*PI/camera_position[2] + FOV/2.0) * lambda; 

    for i in 0..data.0.len() {

        let point = [data.0[i][0], data.0[i][1], data.0[i][2]];

        let x0 = (point[0] - camera_position[0]) * (move_z).tan();
        let y0 = (point[1] - camera_position[1]) * (move_z).tan(); 
        let z0 = point[2] - camera_position[2];

        data.0[i][0] = x0;
        data.0[i][1] = y0;
        data.0[i][2] = z0;

    }   

    
}