use image::{open, DynamicImage};

fn main() {
    let path = format!(
        "{}{}",
        std::env::current_dir().unwrap().display(),
        "/tests/small_car.png"
    );
    let data = get_pixels(&path);
    let test = unsafe { roux::test_window(data.len() as u32, &data[0] as *const u8, 360, 202, 10, 10) };
    println!("{} Clicks!", test);
}

fn get_pixels(path: &str) -> Vec<u8> {
    let rgba = open(path).unwrap().into_rgba8();
    DynamicImage::ImageRgba8(rgba).into_luma8().into_raw()
}

#[test]
fn test_entropy() {
    let test_arr: [u8; 5] = [0, 1, 2, 3, 4];
    let entropy = unsafe { roux::get_entropy(test_arr.len() as u32, &test_arr as *const u8) };
    assert_eq!(2.321928094887362, entropy);
}