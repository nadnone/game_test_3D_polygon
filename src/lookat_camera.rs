use crate::{maths_vectors_helper::{normaliser, soustraction_vectors, produit_vectoriel, multiply_matrix4}, transformations::rotate};


pub struct Camera {
    matrix: [[f32; 4]; 4]
}

impl Camera {
    
    pub fn place(from: [f32; 3], to: [f32; 3], random_up: [f32; 3]) -> Camera
    {


        /* help found there
            
            - https://learnopengl.com/Getting-started/Camera
            - https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function/framing-lookat-function.html
        */

        let forward = normaliser(soustraction_vectors(from, to));
        let right = produit_vectoriel(random_up, forward);
        let new_up = produit_vectoriel(forward, right);
        

        let m_rslt = [
            [right[0], right[1], right[2], 0.],
            [new_up[0], new_up[1], new_up[2], 0.],
            [forward[0], forward[1], forward[2], 0.],
            [from[0], from[1], from[2], 1.],
        ];

        return Self { 
            matrix: m_rslt
        }


    }


    pub fn get_cam_matrix(&self) -> [[f32; 4]; 4]
    {
        return self.matrix;
    }


}