use std::fs;

pub struct JPEG {
    
}


pub fn read_jpeg_file(file_name:String) -> JPEG {
    let jpeg: JPEG = JPEG {};
    let image_data = fs::read(file_name).expect("Could not read image file");

    return jpeg;
}