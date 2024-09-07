use crate::{random_double, random_int, Point3, Scalar};

const POINT_COUNT: u32 = 256;


pub struct Perlin {
    pub random_floats: Vec<Scalar>,

    pub perm_x: Vec<u32>,
    pub perm_y: Vec<u32>,
    pub perm_z: Vec<u32>,

}

fn permute(ints: &mut Vec<u32>, count: u32) {
    for i in 0..count {
        let target = random_int(0..count);
        ints.swap(target as usize, i as usize);
    }
}

fn perlin_generate_perm() -> Vec<u32> {
    let mut ints: Vec<u32> = (0..POINT_COUNT)
    .map(|i| i) // Convert each index to f64
    .collect();  

    permute(&mut ints, POINT_COUNT);
    ints
}

impl Perlin {
    pub fn new() -> Perlin {
        let random_floats: Vec<Scalar> = (0..POINT_COUNT)
        .map(|_| random_double()) // Convert each index to f64
        .collect();  
        Perlin {  
            random_floats,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn cell_noise(&self, p: Point3) -> Scalar {
        let i = (((4.0 * p.x())).rem_euclid(POINT_COUNT as Scalar) as u32) & (POINT_COUNT-1);
        let j = (((4.0 * p.y())).rem_euclid(POINT_COUNT as Scalar) as u32) & (POINT_COUNT-1);
        let k = (((4.0 * p.z())).rem_euclid(POINT_COUNT as Scalar) as u32) & (POINT_COUNT-1);

        self.random_floats[ (self.perm_x[i as usize] ^ self.perm_x[j as usize] ^ self.perm_x[k as usize]) as usize ]  
    }
}