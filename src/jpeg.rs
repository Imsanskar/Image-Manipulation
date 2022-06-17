use std::fs;

pub struct JPEG {
    
}


pub fn read_jpeg_file(file_name:String) -> JPEG {
    let jpeg: JPEG = JPEG {};
    let image_data = fs::read(file_name).expect("Could not read image file");

    let baseline_dct = image_data[0xC4];
    let progressive_dct = image_data[0xFF << 8 | 0xC2];

    println!("{} {}", baseline_dct, progressive_dct);
    return jpeg;
}