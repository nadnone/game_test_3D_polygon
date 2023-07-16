use crate::{maths_vectors_helper::*, lookat_camera::Camera};

pub fn shader(camera: &Camera, normals: &Vec<[f32; 3]>, v0: [f32; 3], color_data: &Vec<[f32; 3]>, i: usize) -> [u8; 3]
{
    /*
         /!\ code entièrement pondu par ChatGPT 
    */

    let light_pos = [100.0, 100.0, 100.0];
    let ambient_intensity = 0.2;
    let diffuse_intensity = 0.7;
    let specular_intensity = 1.0;
    let shininess = 32.0;

    let cam_pos = normaliser(camera.get_cam_pos());
    let norm = normaliser(normals[i]);

    // Calcul de la direction de la vue
    let view_dir = normaliser(soustr_vec(cam_pos, v0));

    // Calcul de la direction de la lumière
    let light_dir = normaliser(soustr_vec(light_pos, v0));

    // Calcul de la composante ambiante
    let ambient = scale(color_data[i], ambient_intensity);

    // Calcul de la composante diffuse
    let diff = produit_scalair(norm, light_dir).max(0.0);
    let diffuse = scale(color_data[i], diffuse_intensity * diff);

    // Calcul de la direction de réflexion
    let reflect_dir = reflect(light_dir, norm);

    // Calcul de la composante spéculaire
    let spec = produit_scalair(reflect_dir, view_dir).max(0.0).powf(shininess);
    let specular = scale([1.0, 1.0, 1.0], specular_intensity * spec);

    // Calcul de la couleur finale
    let total = add_vec(add_vec(ambient, diffuse), specular);

    // Conversion en valeurs entières pour l'affichage
    let r_out = (255.0 * total[0]).min(255.0) as u8;
    let g_out = (255.0 * total[1]).min(255.0) as u8;
    let b_out = (255.0 * total[2]).min(255.0) as u8;

    return [r_out, g_out, b_out];
}

fn reflect(dir: [f32; 3], normal: [f32; 3]) -> [f32; 3] {

    let dot_product = produit_scalair(dir, normal);

    let reflected = soustr_vec(dir, scale(normal, 2.0 * dot_product));

    return normaliser(reflected);
}
