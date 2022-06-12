use std::fs;
use std::ops::{BitOr, Shl};
use std::convert::{From};


fn get_value_from_array <T: BitOr<Output = T> + Shl<Output = T> + From<u32> + From<u8>> (a:&Vec<u8>, index:usize) -> T {

    return  T::from(a[index + 3]) << T::from(24 as u32) | 
            T::from(a[index + 2]) << T::from(16 as u32) | 
            T::from(a[index + 1]) << T::from(8 as u32) | 
            T::from(a[index]);
}


pub fn convert_image_to_jpeg(bmp_filename: String, jpeg_filename: String) {
    let image_pixels = fs::read(bmp_filename).expect("Could not read image file");
    
    let width: u32 = get_value_from_array(&image_pixels, 18);
    let temp_height: i32 = get_value_from_array::<u32>(&image_pixels, 22) as i32;
    let mut bottom_to_top: bool;

    let mut height = if temp_height < 0 {
        bottom_to_top = false;
        - temp_height as u32
    } else {
        bottom_to_top = true;
        temp_height as u32
    };

    let mut pixel_position: u32 = get_value_from_array(&image_pixels, 10);

    let x = 0;
}