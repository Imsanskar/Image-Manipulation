mod read_image;
fn main() {
    let s = String::from("test.jpg");
    read_image::read_jpeg(s);
}
