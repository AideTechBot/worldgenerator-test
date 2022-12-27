use noise::{Fbm, NoiseFn, Perlin};

#[derive(Copy, Clone)]
pub struct MapConfig {
    // These size numbers represent the number of chunks not voxels
    // so each of then is n * 256 voxels
    pub x: usize,
    pub y: usize,
    pub z: usize,

    pub seed: u32,
}

const MODEL_FLOAT_MAX: f64 = u8::MAX as f64;
const MODEL_USIZE_MAX: usize = u8::MAX as usize;

// TODO: make this less magicky and play around with adding other functions to it
// also clamping it so it doesn't panic when shit goes out of bounds

pub fn build_map(config: MapConfig) -> Vec<Vec<Vec<Option<u8>>>> {
    let MapConfig { x, y, z, seed } = config;
    let mut result = vec![vec![vec![None; z * 256]; y * 256]; x * 256];
    let mut perlin = Fbm::<Perlin>::new(seed);
    perlin.octaves = 6;

    for i in 0..(x * MODEL_USIZE_MAX) {
        for j in 0..(y * MODEL_USIZE_MAX) {
            // for k in 0..(z * MODEL_USIZE_MAX) {
            let coords = [
                i as f64 / MODEL_FLOAT_MAX,
                j as f64 / MODEL_FLOAT_MAX,
                // k as f64 / SCALING_FACTOR,
            ];
            let SQRT_POINT_FIVE: f64 = f64::sqrt(6.0 / 4.0);
            println!("{:#?}", perlin.get(coords) + SQRT_POINT_FIVE);
            let k_max = ((perlin.get(coords) + SQRT_POINT_FIVE) * MODEL_FLOAT_MAX / 3.0) as usize;
            for k in 0..k_max {
                let colour = (k as f64 / 10.0);
                result[i][j][k] = Some(colour as u8);
            }
            // }
        }
    }

    result
}
