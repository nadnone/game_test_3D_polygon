use easy_gltf::{self, model::Mode};

pub struct GLTFLoader;

impl GLTFLoader {


    pub fn load(filename: &str) -> Vec<(Vec<[f32; 6]>, Vec<[f32; 3]>, Vec<[f32; 3]>)>
    {


        let mut models = Vec::new();

        let mut models_points = Vec::new();
        let mut models_normals = Vec::new();
        let mut models_colors = Vec::new();


        let scenes = easy_gltf::load(filename).unwrap();


        for scene in scenes {
        

                for model in scene.models {
                    
                    if model.mode() == Mode::Triangles
                    {

                        for vertex in model.triangles().unwrap() {
                            
                            for j in 0..3 {

                                let mut x = vertex[j].position.x;
                                let mut y = vertex[j].position.y;
                                let mut z = vertex[j].position.z;
                                
                                models_points.push([
                                            x, y, z,
                                            x, y ,z // double parce-que transformations
                                        ]);

                                x = vertex[j].normal.x;
                                y = vertex[j].normal.y;
                                z = vertex[j].normal.z;

                                models_normals.push([x, y, z]);


                                let color = model.material().get_base_color(vertex[j].tex_coords);

                                models_colors.push([color.x, color.y, color.z]);
                            }
                        }


                        
                    }


                }
                models.push((models_points.clone(), models_normals.clone(), models_colors.clone()));

                models_points.clear();
                models_normals.clear();
                models_colors.clear();
        }
    
        return models;

    }

}