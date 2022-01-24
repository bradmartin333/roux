#[macro_use]
extern crate glium;

use std::fs;

mod window;

#[no_mangle]
pub extern "C" fn test_window(
    _size: u32,
    _array_pointer: *const u8,
    wid: u32,
    hgt: u32,
    start_x: u32,
    start_y: u32,
) -> u32 {
    let mut tile_size: f32 = window::MIN_TILE_SIZE * 2.0;
    window::test(
        wid as f64,
        hgt as f64,
        start_x as f64,
        start_y as f64,
        &mut tile_size,
    );
    tile_size as u32
}

#[no_mangle]
pub extern "C" fn test_tiles(size: u32, array_pointer: *const u8, stride: u32, tile_size: u32) {
    let data = unsafe { std::slice::from_raw_parts(array_pointer, size as usize) };
    let mut output = String::default();
    for j in (0..(data.len() as u32 / stride)).step_by(tile_size as usize) {
        for i in (0..stride).step_by(tile_size as usize) {
            let mut entropy = 0.0;
            let mut counts = [0; 256];

            for k in 0..tile_size {
                for l in 0..tile_size {
                    let idx = ((j + l) * stride) + (i + k);
                    if data.len() > idx as usize {
                        let val: u8 = data[(((j + l) * stride) + (i + k)) as usize];
                        counts[val as usize] += 1;
                    }
                }
            }

            for &count in &counts {
                if count == 0 {
                    continue;
                }
                let p: f64 = (count as f64) / (data.len() as f64);
                entropy -= p * p.log(2.0);
            }

            output.push_str(&format!("{}\t", entropy));
        }
        output.push_str("\n");
    }
    fs::write(r"C:\Users\delta\Desktop\test_tiles.txt", output).unwrap();
}

#[no_mangle]
pub extern "C" fn get_entropy(size: u32, array_pointer: *const u8) -> f64 {
    let data = unsafe { std::slice::from_raw_parts(array_pointer, size as usize) };
    let mut entropy = 0.0;
    let mut counts = [0; 256];

    for &d in data {
        counts[d as usize] += 1;
    }

    for &count in &counts {
        if count == 0 {
            continue;
        }
        let p: f64 = (count as f64) / (data.len() as f64);
        entropy -= p * p.log(2.0);
    }

    entropy
}
