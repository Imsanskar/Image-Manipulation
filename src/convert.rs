use std::fs;
use std::ops::{BitOr, Shl};
use std::convert::{From};

fn get_value_from_array <T: BitOr<Output = T> + Shl<Output = T> + From<u32> + From<u8>> (a:&Vec<u8>, index:usize) -> T {

    return  T::from(a[index + 3]) << T::from(24 as u32) | 
            T::from(a[index + 2]) << T::from(16 as u32) | 
            T::from(a[index + 1]) << T::from(8 as u32) | 
            T::from(a[index]);
}


struct BitMapHeader {
    bits_per_pixel: u8,
    compression: u8,
    planes: u8,
    width: u32,
    height: u32,
    bottom_to_top: bool,
    pixel_position: u32,
    size: u32,
    reserved_1: u16,
    reserved_2: u16,
}


pub fn convert_image_to_jpeg(bmp_filename: String, jpeg_filename: String) {
    let image_pixels = fs::read(bmp_filename).expect("Could not read image file");
    let len = image_pixels.len();
    
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
    let mut bits_per_pixel: u32 = get_value_from_array(&image_pixels, 28);
    let mut x = pixel_position as usize;

    let mut blue_image: Vec<u8> = Vec::new();
    let mut green_image: Vec<u8> = Vec::new();
    let mut red_image: Vec<u8> = Vec::new();

    for i in 0..pixel_position {
        let pixel: u8 = image_pixels[i as usize] as u8;
        blue_image.push(pixel);
        green_image.push(pixel);
        red_image.push(pixel);
    }

    while (x + 3) < len {
        let blue_pixel = image_pixels[x];
        x += 1;
        let green_pixel = image_pixels[x + 1];
        x += 1;
        let red_pixel = image_pixels[x + 2];
        x += 1;

        blue_image.push(blue_pixel);
        blue_image.push(0);
        blue_image.push(0);


        green_image.push(0);
        green_image.push(green_pixel);
        green_image.push(0);


        red_image.push(0);
        red_image.push(0);
        red_image.push(red_pixel);
    }


    std::fs::write("red.bmp", red_image);
    std::fs::write("green.bmp", green_image);
    std::fs::write("blue.bmp", blue_image);
}