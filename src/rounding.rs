pub use f32;

pub fn round_dp(number: &f32, decimal_places: &usize) -> f32 {
    /* Rounds number to a number of decimal places */
    ((*number) * 10_f32.powi((*decimal_places) as i32)).round() / 10_f32.powi((*decimal_places) as i32) 
}

pub trait Roundable {
    fn round_dp(&self, decimal_places: &usize) -> f32;
}

impl Roundable for f32 {
    fn round_dp(&self, decimal_places: &usize) -> f32 {
        /* Rounds self to a number of decimal places */
        ((*self) * 10_f32.powi((*decimal_places) as i32)).round() / 10_f32.powi((*decimal_places) as i32)
    }
}