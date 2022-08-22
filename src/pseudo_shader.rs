use crate::{maths_vectors_helper::*, controls::EventControls};

pub fn shader(player_event: &EventControls, normals: &Vec<[f32; 3]>, v0: [f32; 3], color_data: &Vec<[f32; 3]>, i: usize) -> [u8; 3]
{

    let cam_pos = normaliser(player_event.get_pos_camera());
    let _cam_dir = normaliser(soustraction_vectors(cam_pos, v0));


    let norm = normaliser(normals[i]);
    let light_pos = normaliser([100.0, 100.0, 100.0]);



    // Lambert diffuse model
    let intensity = produit_scalair(norm, light_pos); // N * L
    let diffuse = scalair(color_data[i], intensity);


    // TODO specular (or not)

    let total = diffuse;


    let r_out = (255. * total[0]).abs() as u8;
    let g_out = (255. * total[1]).abs() as u8;
    let b_out = (255. * total[2]).abs() as u8;


    return [r_out, g_out, b_out];
}

