#[derive(Debug)]
pub struct TargetResolution {
    pub x: u32,
    pub y: u32,
    pub aspect_ratio: (u32, u32),
    pub aspect_ratio_float: f64,
}

impl TargetResolution {
    pub fn new(x: u32, y: u32) -> TargetResolution {
        let lowest_denominator = gcd(x, y);
        TargetResolution {
            x: x,
            y: y,
            aspect_ratio: (x / lowest_denominator, y / lowest_denominator),
            aspect_ratio_float: x as f64 / y as f64,
        }
    }
}

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

fn gcd(x: u32, y: u32) -> u32 {
    let mut numerator = x;
    let mut modulo = x % y;
    while modulo != 0 {
        let cache = modulo; // this lives on stack
        modulo = numerator % modulo;
        numerator = cache;
    }
    return numerator;
}