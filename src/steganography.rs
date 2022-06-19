pub use crate::bmp;

pub fn least_bit_steganography_encode(bitmap_image: &bmp::BitMap, message: String) -> bmp::BitMap {
    let mut image_with_data = bmp::BitMap::default();
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

pub fn least_bit_steganography_decode(image: &bmp::BitMap) -> String {
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
    for _ in 1..(message_length + 1) {
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

    return message;
}