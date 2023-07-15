use crate::constants::*;
use crate::maths_vectors_helper::{mat4_multiply_vec4, multiply_matrix4};


/// ```
/// data: (points, (normals, phon_data)), 
pub fn projection(data: &mut (Vec<[f32; 4]>, Vec<[f32; 3]>, Vec<[f32; 3]>), view_matrix: [[f32; 4]; 4])
{

    /*
        comprendre la matrice de projection
        - https://youtu.be/U0_ONQQ5ZNM
        - https://youtu.be/Do_vEjd6gF0?list=PLYnrabpSIM-97qGEeOWnxZBqvR_zwjWoo
    */

    let znear = 1.;
    let zfar = 1000.;

    for i in 0..data.0.len() {

        let point = data.0[i]; // une coordonnnées d'un triangle [x,y,z]

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


        // application de la caméra
        let cam_proj = multiply_matrix4( m_proj, view_matrix);
        
        // multiplication avec les coordonnées
        let mut rslt = mat4_multiply_vec4(cam_proj, point);

        // projection
        if rslt[3] != 0. 
        {
            rslt[0] /= rslt[3];   // x / w
            rslt[1] /= rslt[3];   // y / w
            rslt[2] /= rslt[3];   // z / w
        }


        data.0[i] = rslt;

    }   

    
}