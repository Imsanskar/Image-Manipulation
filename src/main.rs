pub mod bmp;
pub mod jpeg;
pub mod steganography;

fn main() {
    let image = bmp::read_bitmap("image.bmp");
    steganography::least_bit_steganography_encode(&image, String::from("Hello there")).save_bitmap("stegano.bmp");

    let bitmap = bmp::read_bitmap("stegano.bmp");
    let message = steganography::least_bit_steganography_decode(&bitmap);

    println!("{} \n{}", message, message.len());
}

