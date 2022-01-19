mod canvas;
mod color;
mod image;
mod input;
mod window;

#[no_mangle]
pub unsafe extern "C" fn test_window(
    size: u32,
    array_pointer: *const u8,
    wid: i32,
    hgt: i32,
    x: i32,
    y: i32,
) -> u32 {
    let data = std::slice::from_raw_parts(array_pointer, size as usize);
    let zero: Box<u32> = Box::new(0);
    let pointer = Box::into_raw(zero);
    window::test(pointer, data, wid, hgt, x, y);
    let num_clicks = Box::from_raw(pointer);
    *num_clicks
}

#[no_mangle]
pub unsafe extern "C" fn get_entropy(size: u32, array_pointer: *const u8) -> f64 {
    let data = std::slice::from_raw_parts(array_pointer, size as usize);
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
