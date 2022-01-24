#[macro_use]
extern crate glium;

mod window;

#[no_mangle]
pub extern "C" fn test_window(
    size: u32,
    array_pointer: *const u8,
    wid: u32,
    hgt: u32,
    start_x: u32,
    start_y: u32,
) {
    window::test(
        size,
        array_pointer,
        wid as f64,
        hgt as f64,
        start_x as f64,
        start_y as f64,
    );
}
