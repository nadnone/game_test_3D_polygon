use crate::maths_vectors_helper::multiply_matrix;

pub fn rotate(a: &Vec<[f32; 4]>, angle: f32, axe: char) -> Vec<[f32; 4]>
{

    let cos = angle.cos();
    let sin = angle.sin();

    let matrix_rot;

    if axe == 'y'
    {
        matrix_rot = [
            [cos, 0.0, sin],
            [0.0, 1.0, 0.0],
            [-sin, 0.0, cos]
        ];
    }
    else if axe == 'x'
    {
        matrix_rot = [
            [1.0, 0.0, 0.0],
            [0.0, cos, -sin],
            [0.0, sin, cos]
        ];
    }
    else // z
    {
        matrix_rot = [
            [cos, -sin, 0.0],
            [sin, cos, 0.0],
            [0.0, 0.0, 1.0]
        ];
    }




    let mut m_out = Vec::new();

    for p in (0..a.len()).step_by(3) {

        let mut kernel: [[f32; 3]; 3] = [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0]
        ];

        for i in 0..3
        {
            kernel[0][i] = a[p + 0][i];
            kernel[1][i] = a[p + 1][i];
            kernel[2][i] = a[p + 2][i];
        }
        
        let res = multiply_matrix(kernel, matrix_rot);



        for i in 0..3 {

            m_out.push(
                [
                    res[i][0], 
                    res[i][1], 
                    res[i][2],
                    1.
                ]
            );

        }
       
    }   


    return m_out;
}

pub fn translate(a: &Vec<[f32; 4]>, translation: [f32; 3]) -> Vec<[f32; 4]>
{


    let mut m_out = a.clone();

    for p in 0..a.len() {

        m_out[p][0] = a[p][0] + translation[0];
        m_out[p][1] = a[p][1] + translation[1];
        m_out[p][2] = a[p][2] + translation[2];
       
    }   

    return m_out;
}


pub fn scale(a: &Vec<[f32; 4]>, factor: f32) -> Vec<[f32; 4]>
{
    let mut m_out = a.clone();

    for i in 0..a.len() {

        for j in 0..3 {
            
            m_out[i][j] = a[i][j] * factor;

        }

    }

    return m_out;
}

pub fn transform(a: &Vec<[f32; 4]>, transform_matrix: [[f32; 4]; 4]) -> Vec<[f32; 4]>
{

    let mut m_out = a.clone();

    for el in 0..a.len() {
        

        for i in 0..4 {
            for j in 0..4 {
                
                // multiplication mat4 sur vec4
                m_out[el][i] += transform_matrix[i][j] * a[el][j];
            }
        }

    }



    return m_out;

}
