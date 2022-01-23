use image::GenericImageView;

fn main() {
    let (wid, hgt, _data) = get_pixels(&get_test_path());
    unsafe { roux::test_window(0, 0 as *const u8, wid as u32, hgt as u32) };
}

fn get_test_path() -> String {
    format!(
        "{}{}",
        std::env::current_dir().unwrap().display(),
        "/tests/small_car.png"
    )
}

fn get_pixels(path: &str) -> (u32, u32, Vec<u8>) {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    let data = img.into_luma8().into_raw();
    (width, height, data)
}

#[test]
fn test_entropy() {
    let test_arr: [u8; 5] = [0, 1, 2, 3, 4];
    let entropy = unsafe { roux::get_entropy(test_arr.len() as u32, &test_arr as *const u8) };
    assert_eq!(2.321928094887362, entropy);
}