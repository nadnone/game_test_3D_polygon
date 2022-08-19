use crate::maths_vectors_helper::*;

pub fn shader_phong(normals: &Vec<[f32; 3]>, v0: [f32; 3], phong_data: &(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, f32), i: usize) -> [u8; 3]
{
    // diffuse
    let norm = normals[i];

    let light = [1.,1.,1.];
    
    let light_dir = normaliser(
        scalair(
             soustraction_vectors([0.0, 400.0, -50.0], v0)
             , -1.0)
    
    );


    let diff = produit_scalair(norm, light_dir);
    let diffuse = scalair(phong_data.1[0], diff); 

    // ambient                        
    let ambient = produit_vectoriel(light, phong_data.0[0]);


    // specular
    let cam_pos = normaliser([0.0, 0.0, 200.0]);
    let cam_dir = normaliser(soustraction_vectors(cam_pos, v0));
    let reflect_dir = reflect(light_dir, norm);

    let angle = produit_scalair(cam_dir,  reflect_dir);

    let mut specular = scalair(light, angle);
            specular = produit_vectoriel(specular, phong_data.2[0]);    
            specular = scalair(specular, phong_data.3);


            

    let mut phong = addition_vectors(diffuse, ambient);
        phong = addition_vectors(phong, specular);



    let r_out = (255. * phong[0]) as u8;
    let g_out = (255. * phong[1]) as u8;
    let b_out = (255. * phong[2]) as u8;


    return [r_out, g_out, b_out];
}

fn reflect(light_dir: [f32; 3], normal: [f32; 3]) -> [f32; 3]
{
    let a = produit_vectoriel(normal, scalair(light_dir, -1.0));

    let b = produit_vectoriel(normal, a);
    
    let total = produit_vectoriel(normal, b);

    return scalair(total, -1.);
}