use crate::lib::resolution::crop_resolution;
use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType;

use super::resolution::TargetResolution;

pub fn rescale(original_img: &DynamicImage, target_resolution: TargetResolution, file_name: &String) -> () {
    let crop_resolution = crop_resolution(original_img.dimensions(), &target_resolution);

    let cropped_img = original_img.crop_imm(crop_resolution.center_coords.0, crop_resolution.center_coords.1, crop_resolution.x, crop_resolution.y);
    let resized_img = cropped_img.resize(target_resolution.x, target_resolution.y, FilterType::Gaussian);

    let target_file_name = format!("{}-{}x{}", file_name, target_resolution.x, target_resolution.y);
    let target_file_path = format!(".test/output/{}.png", target_file_name);
    resized_img.save(target_file_path).unwrap();
    println!("COMPLETED TASK with {:?}", target_resolution);
}