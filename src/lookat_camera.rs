use crate::maths_vectors_helper::{normaliser, soustraction_vectors, produit_vectoriel, multiply_matrix4, _addition_vectors, mat3_multiply_vec3, produit_scalair};


pub struct Camera {
    matrix: [[f32; 4]; 4],
    forward: [f32; 3],
    right: [f32; 3],
    up: [f32; 3]
}

impl Camera {
    
    pub fn place(from: [f32; 3], to: [f32; 3], random_up: [f32; 3]) -> Camera
    {


        /* help found there
            
            - https://learnopengl.com/Getting-started/Camera
            - https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function/framing-lookat-function.html

            rotate the camera
            - https://medium.com/@carmencincotti/lets-look-at-magic-lookat-matrices-c77e53ebdf78
        
        */

        let forward = normaliser(soustraction_vectors(from, to));
        let right = normaliser(produit_vectoriel(random_up, forward));
        let new_up = normaliser(produit_vectoriel(forward, right));
        

        let tx = produit_scalair(from, right);
        let ty = produit_scalair(from, new_up);
        let tz = produit_scalair(from, forward);

        let view_matrix = [
            [right[0], new_up[0], forward[0], tx],
            [right[1], new_up[1], forward[1], ty],
            [right[2], new_up[2], forward[2], tz],
            [0., 0., 0., 1.],
        ];

        return Self { 
            matrix: view_matrix,
            forward: forward,
            right: right,
            up: new_up
        }


    }

    pub fn rotate_y(&mut self, angle: f32)
    {   

        let cos = angle.cos();
        let sin = angle.sin();

        let matrix_rot = [
            [cos, 0., sin, 0.],
            [0., 1., 0., 0.],
            [-sin, 0., cos, 0.],
            [0., 0., 0., 1.],
        ];

        self.matrix = multiply_matrix4(matrix_rot, self.matrix);


    }


    pub fn get_cam_matrix(&self) -> [[f32; 4]; 4]
    {
        return self.matrix;
    }


}