#[derive(Default, Debug)]
pub struct FPixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    // pub alpha: f64,
}

impl FPixel {
    pub fn to_upixel(&self) -> UPixel {
        fn linear_to_gama(linear_component: f64) -> f64 {
            if linear_component > 0.0 {
                linear_component.sqrt()
            } else {
                0.0
            }
        }

        fn clamp_intensity(val: f64) -> f64 {
            val.clamp(0.0, 0.999)
        }

        let &Self { red, green, blue } = self;
        let red = linear_to_gama(red);
        let green = linear_to_gama(green);
        let blue = linear_to_gama(blue);
        // let alpha = alpha;

        UPixel {
            red: (256.0 * clamp_intensity(red)) as u8,
            green: (256.0 * clamp_intensity(green)) as u8,
            blue: (256.0 * clamp_intensity(blue)) as u8,
            // alpha: (256.0 * clamp_intensity(alpha)) as u8,
        }
    }
}

#[derive(Default, Debug)]
pub struct UPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    // pub alpha: u8,
}
