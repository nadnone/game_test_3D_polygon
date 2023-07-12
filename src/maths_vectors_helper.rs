pub fn soustraction_vectors(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
{

    let mut res: [f32; 3] = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        
        res[i] = a[i] - b[i];
    }
    
    return res;
}

pub fn _addition_vectors(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
{

    let mut res: [f32; 3] = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        
        res[i] = a[i] + b[i];
    }
    
    return res;
}


pub fn norme(a: [f32; 3]) -> f32
{
    return (a[0].powf(2.0) + a[1].powf(2.0) + a[2].powf(2.0)).sqrt();
}

pub fn normaliser(a: [f32; 3]) -> [f32; 3]
{
    return scale(a, 1.0/norme(a));
}


pub fn produit_scalair(a: [f32; 3], b :[f32; 3]) -> f32
{
     return (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2]);
}



pub fn scale(a: [f32; 3], factor: f32) -> [f32; 3]
{
    let mut res = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        res[i] = a[i] * factor;    
    }

    return res;
}


pub fn multiply_vectors(a: [[f32; 3]; 3], b: [[f32; 3]; 3]) -> [[f32; 3]; 3]
{
    
    let mut res: [[f32; 3]; 3] = [
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0]
    ];
    
    for i in 0..3 {

        for j in 0..3 {
            
            for k in 0..3 {

                res[i][j] += a[i][k] * b[k][j];

            }

        }

    }

    return res;
}


pub fn mat4_multiply_vec4(mat4: [[f32; 4]; 4], vector: [f32; 4]) -> [f32; 4]
{
    
    let mut rslt = [0., 0., 0., 0.];

    for i in 0..4 {
        for j in 0..4 {

                // https://mbernste.github.io/posts/matrix_vector_mult/
                rslt[j] += mat4[i][j] * vector[j];
        }
    }

   return rslt;
    

}