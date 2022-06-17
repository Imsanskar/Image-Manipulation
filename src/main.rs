pub mod bmp;
pub mod jpeg;

fn main() {
    let image = bmp::read_bitmap("image.bmp");
    bmp::least_bit_steganography_encode(&image, String::from("
    Steganography is the technique of hiding secret data within an ordinary, non-secret, file or message in order to avoid detection; the secret data is then extracted at its destination. The use of steganography can be combined with encryption as an extra step for hiding or protecting data. The word steganography is derived from the Greek words steganos (meaning hidden or covered) and the Greek root graph (meaning to write).
    Steganography can be used to conceal almost any type of digital content, including text, image, video or audio content; the data to be hidden can be hidden inside almost any other type of digital content. The content to be concealed through steganography -- called hidden text -- is often encrypted before being incorporated into the innocuous-seeming cover text file or data stream. If not encrypted, the hidden text is commonly processed in some way in order to increase the difficulty of detecting the secret content.
    ")).save_bitmap("stegano.bmp");

    let bitmap = bmp::read_bitmap("stegano.bmp");
    println!("{}", bmp::least_bit_steganography_decode(&bitmap));
}

