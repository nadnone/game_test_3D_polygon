use crate::maths_vectors_helper::{normaliser, soustraction_vectors, produit_vectoriel, produit_scalair};


pub struct Camera;

impl Camera {
    
    pub fn place(from: [f32; 3], to: [f32; 3], random_up: [f32; 3]) -> [[f32; 4]; 4]
    {


        /* help found here
            
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
            [right[0], new_up[0], forward[0], -tx],
            [right[1], new_up[1], forward[1], -ty],
            [right[2], new_up[2], forward[2], -tz],
            [0., 0., 0., 1.],
        ];

        return view_matrix;


    }

}