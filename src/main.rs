mod convert;

fn main() {
    let s = String::from("image.bmp");
    convert::convert_image_to_jpeg(s, String::from("image.jpg"));
}
