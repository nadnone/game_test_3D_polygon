use crate::constants::*;
use crate::controls::EventControls;
use crate::maths_vectors_helper::mat4_multiply_vec4;


/// ```
/// data: (points, (normals, phon_data)), 
pub fn projection(data: &mut (Vec<[f32; 4]>, Vec<[f32; 3]>, Vec<[f32; 3]>), player_event: &EventControls)
{

    /*
        comprendre la matrice de projection
        - https://youtu.be/U0_ONQQ5ZNM
        - https://youtu.be/Do_vEjd6gF0?list=PLYnrabpSIM-97qGEeOWnxZBqvR_zwjWoo
    */

    // déplacement de la caméra
    let camera_position = player_event.get_pos_camera(); // coordonnées de la caméra [x,y,z]
    
    let znear = camera_position[2] + 1.;
    let zfar = camera_position[2] + 1000.;

    for i in 0..data.0.len() {

        let mut point = data.0[i]; // une coordonnnées d'un triangle [x,y,z]


        // mouvement de la caméra dans le monde
        point[0] += camera_position[0];
        point[1] += camera_position[1];
        point[2] += camera_position[2];

        // matrice de projection
        let mut m_proj = [
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.]
        ];

        // remplissage de la matrice de projection
        m_proj[0][0] = HEIGHT/WIDTH * (1. / (FOV/2.).tan());
        m_proj[1][1] = 1. / (FOV/2.).tan();
        m_proj[2][2] = zfar / (zfar - znear);
        m_proj[3][3] = (-zfar * znear) / (zfar - znear);
        m_proj[3][2] = 1.0;


        // multiplication avec les coordonnées
        let mut rslt = mat4_multiply_vec4(m_proj, point);

        // projection
        if rslt[3] != 0. 
        {
            rslt[0] /= rslt[3];   // x / w
            rslt[1] /= rslt[3];   // y / w
            rslt[1] /= rslt[3];   // z / w
        }


        data.0[i] = rslt;

    }   

    
}