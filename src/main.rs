use std::fs::File;

mod generate;
mod vox_writer;

fn main() {
    println!("Staring generation...");

    let config = generate::MapConfig {
        x: 1,
        y: 1,
        z: 1,
        seed: 2,
    };

    let map = generate::build_map(config);

    println!("Translating to .vox...");
    let vox = vox_writer::map_to_vox(map, config);
    let path = "src/resources/testing.vox";
    let mut buffer = File::create(path).unwrap();

    println!("Writing to {}...", path);
    vox.write_vox(&mut buffer).unwrap();

    println!("Done.");
}
