mod lib;
use lib::resolution::Resolution;
use lib::image_processing;



fn main() {
    let target_resolution = Resolution::new(30, 30);

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("another.jpg").unwrap();

    image_processing::rescale(img, target_resolution);
}
