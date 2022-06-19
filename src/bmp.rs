use std::fs;
use std::ops::{BitOr, Shl};
use std::convert::{From};

fn get_value_from_array <T: BitOr<Output = T> + Shl<Output = T> + From<u32> + From<u8>> (a:&Vec<u8>, index:usize) -> T {
    return  T::from(a[index + 3]) << T::from(24 as u32) | 
            T::from(a[index + 2]) << T::from(16 as u32) | 
            T::from(a[index + 1]) << T::from(8 as u32) | 
            T::from(a[index]);
}


pub struct BitMap {
    pub bits_per_pixel: u8,
    pub compression: u8,
    pub planes: u8,
    pub width: u32,
    pub height: u32,
    pub bottom_to_top: bool,
    pub pixel_position: u32,
    pub size: u32,
    pub reserved_1: u16,
    pub reserved_2: u16,
 
    pub pixels: Vec<u8>
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

    pub fn copy_header(&mut self, image: &BitMap) {
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
    let bottom_to_top: bool;
    let bits_per_pixel: u32 = get_value_from_array(&image_pixels, 28);
    
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

