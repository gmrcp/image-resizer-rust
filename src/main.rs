use image::GenericImageView;
use image::imageops::FilterType;
mod lib;
use lib::resolution::TargetResolution;
use lib::calculate::crop_resolution;

fn main() {
    let target_resolution = TargetResolution::new(30, 30);

    let file_name = "another";
    let file_path = format!(".test/input/{}.jpg", file_name);
    let original_img = image::open(file_path).unwrap();

    let crop_resolution = crop_resolution(original_img.dimensions(), &target_resolution);

    let cropped_img = original_img.crop_imm(crop_resolution.center_coords.0, crop_resolution.center_coords.1, crop_resolution.x, crop_resolution.y);
    let resized_img = cropped_img.resize(target_resolution.x, target_resolution.y, FilterType::Gaussian);

    let target_file_name = format!("{}-{}x{}", file_name, target_resolution.x, target_resolution.y);
    let target_file_path = format!(".test/output/{}.png", target_file_name);
    resized_img.save(target_file_path).unwrap();
}
