use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType;
use super::resolution::Resolution;

pub fn rescale(original_img: DynamicImage, target_resolution: Resolution) {
    let original_resolution = original_img.dimensions();
    let original_ar_float = original_resolution.0 as f64 / original_resolution.1 as f64;

    println!("Target RESOLUTION{:?} {:?}", target_resolution.x, target_resolution.y);
    println!("Current RESOLUTION{:?}", original_resolution);

    println!("Target AR {:?}", target_resolution.aspect_ratio_float);
    println!("Current AR {:?}", original_ar_float);

    let cropped_y: u32;
    let cropped_x: u32;
    if original_ar_float > target_resolution.aspect_ratio_float {
        cropped_y = original_resolution.1 - (original_resolution.1 % target_resolution.aspect_ratio.1);
        cropped_x = target_resolution.aspect_ratio.0 * cropped_y / target_resolution.aspect_ratio.1;
    } else {
        cropped_x = original_resolution.0 - (original_resolution.0 % target_resolution.aspect_ratio.0);
        cropped_y = target_resolution.aspect_ratio.1 * cropped_x / target_resolution.aspect_ratio.0;
    }
    let crop_resolution = (cropped_x, cropped_y);

    println!("crop resolution {:?}", crop_resolution);

    let center_crop_x = original_resolution.0 / 2 - crop_resolution.0 / 2;
    let center_crop_y = original_resolution.1 / 2 - crop_resolution.1 / 2;
    
    let cropped_img = original_img.crop_imm(center_crop_x, center_crop_y, crop_resolution.0, crop_resolution.1);
    println!("CROPPED FIRST {:?}", cropped_img.dimensions());
    let resized_img = cropped_img.resize(target_resolution.x, target_resolution.y, FilterType::Gaussian);
    println!("CROPPED SECOND {:?}", cropped_img.dimensions());
    println!("RESIZED{:?}", resized_img.dimensions());

    resized_img.save("test.png").unwrap();
}