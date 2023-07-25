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

        let mut point = data.0[i]; // une coordonnnées d'un point de triangle [x,y,z]

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

        // division par w à faire avant la multiplication matricielle (merci ChatGPT)
        if point[3] != 0.
        {
            point[0] /= point[3];   // x / w
            point[1] /= point[3];   // y / w
            point[2] /= point[3];   // z / w
        }

        // application de la caméra
        let cam_proj = multiply_matrix4( m_proj, view_matrix);
        
        // multiplication avec les coordonnées
        let rslt = mat4_multiply_vec4(cam_proj, point);


        data.0[i] = rslt;

    }   

    
}