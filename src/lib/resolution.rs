pub struct Resolution {
    pub x: u32,
    pub y: u32,
    pub aspect_ratio: (u32, u32),
    pub aspect_ratio_float: f64,
}

impl Resolution {
    pub fn new(x: u32, y: u32) -> Resolution {
        let lowest_denominator = gcd(x, y);
        Resolution {
            x: x,
            y: y,
            aspect_ratio: (x / lowest_denominator, y / lowest_denominator),
            aspect_ratio_float: x as f64 / y as f64,
        }
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