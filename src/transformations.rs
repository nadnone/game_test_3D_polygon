pub fn rotate(a: &Vec<[f32; 3]>, angle: f32, axe: char) -> Vec<[f32; 3]>
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



        let kernel = [a[p], a[p+1], a[p+2]];

        
        let mut res: [[f32; 3]; 3] = [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0]
        ];

        
        for i in 0..3 {

            for j in 0..3 {
                
                for k in 0..3 {

                    res[i][j] += kernel[i][k] * matrix_rot[k][j];

                }

            }

        }

        m_out.push(res[0]);
        m_out.push(res[1]);
        m_out.push(res[2]);
    }   

   

    return m_out;
}


pub fn translate(a: &Vec<[f32; 3]>, translation: [f32; 3]) -> Vec<[f32; 3]>
{


    let mut m_out = Vec::new();

    for p in 0..a.len() {


        let r0 = a[p][0] + translation[0];
        let r1 = a[p][1] + translation[1];
        let r2 = a[p][2] + translation[2];


        m_out.push([r0, r1, r2]);

       
    }   

   

    return m_out;
}