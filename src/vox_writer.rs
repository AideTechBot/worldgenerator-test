use dot_vox::*;

use crate::generate::MapConfig;

// TODO: refactor this so it does any size and splits up the models and stuff
pub fn map_to_vox(map: Vec<Vec<Vec<Option<u8>>>>, config: MapConfig) -> DotVoxData {
    let MapConfig { x, y, z, seed } = config;
    let mut voxels: Vec<Voxel> = vec![];

    for i in 0..(x * 256) {
        for j in 0..(y * 256) {
            for k in 0..(z * 256) {
                // if map[i][j][k] < 20 {
                match map[i][j][k] {
                    None => (),
                    Some(val) => voxels.push(Voxel {
                        x: i as u8,
                        y: j as u8,
                        z: k as u8,
                        i: val,
                    }),
                }
                // }
            }
        }
    }
    DotVoxData {
        version: 150,
        models: vec![Model {
            size: Size {
                x: (x * 256) as u32,
                y: (y * 256) as u32,
                z: (z * 256) as u32,
            },
            voxels: voxels,
        }],
        palette: DEFAULT_PALETTE.to_vec(),
        materials: (0..256)
            .into_iter()
            .map(|i| Material {
                id: i,
                properties: {
                    let mut map = Dict::new();
                    map.insert("_ior".to_owned(), "0.3".to_owned());
                    map.insert("_spec".to_owned(), "0.5".to_owned());
                    map.insert("_rough".to_owned(), "0.1".to_owned());
                    map.insert("_type".to_owned(), "_diffuse".to_owned());
                    map.insert("_weight".to_owned(), "1".to_owned());
                    map
                },
            })
            .collect(),
        scenes: placeholder::SCENES.to_vec(),
        layers: placeholder::LAYERS.to_vec(),
    }
}
