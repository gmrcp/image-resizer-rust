use super::resolution::TargetResolution;

pub struct CropResolution {
    pub x: u32,
    pub y: u32,
    pub center_coords: (u32, u32),
}

pub fn crop_resolution(original_resolution: (u32, u32), target_resolution: &TargetResolution) -> CropResolution {
    let original_ar_float = original_resolution.0 as f64 / original_resolution.1 as f64;

    let crop_y: u32;
    let crop_x: u32;
    if original_ar_float > target_resolution.aspect_ratio_float {
        crop_y = original_resolution.1 - (original_resolution.1 % target_resolution.aspect_ratio.1);
        crop_x = target_resolution.aspect_ratio.0 * crop_y / target_resolution.aspect_ratio.1;
    } else {
        crop_x = original_resolution.0 - (original_resolution.0 % target_resolution.aspect_ratio.0);
        crop_y = target_resolution.aspect_ratio.1 * crop_x / target_resolution.aspect_ratio.0;
    }

    let crop_center_x = original_resolution.0 / 2 - crop_x / 2;
    let crop_center_y = original_resolution.1 / 2 - crop_y / 2;

    CropResolution {
        x: crop_x,
        y: crop_y,
        center_coords: (crop_center_x, crop_center_y),
    }
}