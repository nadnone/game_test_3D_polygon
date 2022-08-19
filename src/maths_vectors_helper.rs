pub fn soustraction_vectors(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
{

    let mut res: [f32; 3] = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        
        res[i] = a[i] - b[i];
    }
    
    return res;
}

pub fn addition_vectors(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
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
    return scalair(a, 1.0/norme(a));
}

pub fn produit_vectoriel(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
{
    let mut cross = [0.0, 0.0, 0.0];

    cross[0] = (a[1] * b[2]) - (a[2] * b[1]);
    cross[1] = (a[2] * b[0]) - (a[0] * b[2]);
    cross[2] = (a[0] * b[1]) - (a[1] * b[0]);

    return cross;
}

pub fn produit_scalair(a: [f32; 3], b :[f32; 3]) -> f32
{
     return (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2]);
}



pub fn scalair(a: [f32; 3], factor: f32) -> [f32; 3]
{
    let mut res = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        res[i] = a[i] * factor;    
    }

    return res;
}

pub fn _angle_between_vectors(a: [f32;3], b: [f32; 3]) -> f32
{
    return produit_scalair(a, b) / (norme(a) * norme(b));
}

