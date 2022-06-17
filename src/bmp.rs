use std::fs;
use std::ops::{BitOr, Shl};
use std::convert::{From};

fn get_value_from_array <T: BitOr<Output = T> + Shl<Output = T> + From<u32> + From<u8>> (a:&Vec<u8>, index:usize) -> T {
    return  T::from(a[index + 3]) << T::from(24 as u32) | 
            T::from(a[index + 2]) << T::from(16 as u32) | 
            T::from(a[index + 1]) << T::from(8 as u32) | 
            T::from(a[index]);
}

#[derive(Copy, Clone)]
pub struct Color{
    red: u8,
    green: u8,
    blue: u8,
}




pub struct BitMap {
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

    pixels: Vec<u8>
}

impl Default for BitMap {
    fn default() -> BitMap {
        BitMap { 
            bits_per_pixel: 0,
            compression: 0,
            planes: 0,
            width: 0,
            height: 0,
            bottom_to_top: true,
            pixel_position: 0,
            size: 0,
            reserved_1: 0,
            reserved_2: 0,
            pixels: Vec::new()
        }
    }
}

impl BitMap {
    pub fn save_bitmap(&self, file_name: &str) {
        let result = fs::write(file_name, &self.pixels);
    }

    fn copy_header(&mut self, image: &BitMap) {
        self.bits_per_pixel = image.bits_per_pixel;
        self.compression = image.compression;
        self.width = image.width;
        self.height = image.height;
        self.pixel_position = image.pixel_position;
        self.size = image.size;
        self.reserved_1 = image.reserved_1;
        self.reserved_2 = image.reserved_2;
        self.planes = image.planes;

        self.pixels = vec![0; image.pixels.len()];
    } 
}

pub fn read_bitmap(bmp_filename: &str) -> BitMap {
    let image_pixels = fs::read(bmp_filename).expect("Could not read image file");
    let temp_height: i32 = get_value_from_array::<u32>(&image_pixels, 22) as i32;
    let mut bottom_to_top: bool;
    let mut bits_per_pixel: u32 = get_value_from_array(&image_pixels, 28);
    
    // store the data in structs
    let mut bitmap: BitMap = BitMap::default();
    bitmap.bits_per_pixel = bits_per_pixel as u8;
    bitmap.height = if temp_height < 0 {
        bottom_to_top = false;
        - temp_height as u32
    } else {
        bottom_to_top = true;
        temp_height as u32
    };
    bitmap.width = get_value_from_array(&image_pixels, 18);
    bitmap.bottom_to_top = bottom_to_top;
    bitmap.pixel_position = get_value_from_array(&image_pixels, 10);
    bitmap.size = get_value_from_array(&image_pixels, 0x22); // size of the bitmap data

    // let pixels_size = (bitmap.width * bitmap.height* (bits_per_pixel as u32 / 8)) as usize;
    // let pixels_size = image_pixels.len();

    bitmap.pixels = image_pixels;

    return bitmap;
}


pub fn least_bit_steganography_encode(bitmap_image: &BitMap, message: String) -> BitMap {
    let mut image_with_data = BitMap::default();
    image_with_data.copy_header(&bitmap_image);

    let size = bitmap_image.pixels.len();
    let mut message_index:usize = 0;
    let mut i:usize = bitmap_image.pixel_position as usize;
    let characters = message.as_bytes();
    let message_size = characters.len();

    for x in 0..bitmap_image.pixels.len() {
        image_with_data.pixels[x] = bitmap_image.pixels[x];
    }

    // first 16 bytes stores length of the string, size can be maximum of 2^16
    // intially set all the LSB to 0, then OR with the bit to be stored
    // if the bit to be stored is 1 then 0 | 1 = 1 so 1 will be store
    // else if the bit to be store is 0 then 0 | 0 = 0 so 0 will be stored
    for iter in 0..8 {
        image_with_data.pixels[i + iter] &= !1;
        image_with_data.pixels[i + iter] = image_with_data.pixels[i + iter] | (((message_size as u8) >> iter) & 1);
    }
    i += 8;

    // for another 8 bits of data, higher 8 bits of the size 
    for iter in 0..8 {
        image_with_data.pixels[i + iter] &= !1;
        image_with_data.pixels[i + iter] = image_with_data.pixels[i + iter] | ((((message_size >> 8) as u8) >> iter) & 1);
    }
    i += 8;


    while i < size {
        if message_index < message_size {
            let character:u8 = characters[message_index];
            message_index += 1; 
            for iter in 0..8 {
                image_with_data.pixels[i + iter] &= !1;
                image_with_data.pixels[i + iter] = image_with_data.pixels[i + iter] | (((character as u8) >> iter) & 1);
            }
            
            i += 8;
        } else {
            break;
        }
    }

    return image_with_data;
}

pub fn least_bit_steganography_decode(image: &BitMap) -> String {
    let mut message = String::new();

    let mut x = image.pixel_position as usize;
    let message_length = (image.pixels[x] as u32 & 1 ) | 
                            ((image.pixels[x + 1] as u32 & 1) << 1) | 
                            ((image.pixels[x + 2] as u32 & 1) << 2) | 
                            ((image.pixels[x + 3] as u32 & 1) << 3) | 
                            ((image.pixels[x + 4] as u32 & 1) << 4) | 
                            ((image.pixels[x + 5] as u32 & 1) << 5) | 
                            ((image.pixels[x + 6] as u32 & 1) << 6) | 
                            ((image.pixels[x + 7] as u32 & 1) << 7) |
                            ((image.pixels[x + 8] as u32 & 1) << 8) | 
                            ((image.pixels[x + 9] as u32 & 1) << 9) | 
                            ((image.pixels[x + 10] as u32 & 1) << 10) | 
                            ((image.pixels[x + 11] as u32 & 1) << 11) | 
                            ((image.pixels[x + 12] as u32 & 1) << 12) | 
                            ((image.pixels[x + 13] as u32 & 1) << 13) | 
                            ((image.pixels[x + 14] as u32 & 1) << 14) | 
                            ((image.pixels[x + 15] as u32 & 1) << 15); 

    x += 16;
    for i in 1..(message_length + 1) {
        let ascii_value = (image.pixels[x] as u32 & 1 ) | 
                            ((image.pixels[x + 1] as u32 & 1) << 1) | 
                            ((image.pixels[x + 2] as u32 & 1) << 2) | 
                            ((image.pixels[x + 3] as u32 & 1) << 3) | 
                            ((image.pixels[x + 4] as u32 & 1) << 4) | 
                            ((image.pixels[x + 5] as u32 & 1) << 5) | 
                            ((image.pixels[x + 6] as u32 & 1) << 6) | 
                            ((image.pixels[x + 7] as u32 & 1) << 7);

        let character = char::from(ascii_value as u8);
        message.push(character);
        x += 8;
    }

    let x = 0;
    return message;
}