use crate::rounding::*;

const DP: usize = 5;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    fn __clone_xydeg(vec2: &Vec2, degrees: &f32) -> (f32, f32, f32) {
        /* A private method to get x, y, and degree value clones */
        ((*vec2).x.clone(), (*vec2).y.clone(), (*degrees).clone())
    }

    pub fn rotate(&mut self, degrees: &f32) {
        /* Rotate the vector by degrees degrees to 5dp */

        // Create clones to do maths with
        let (x, y, deg) = Vec2::__clone_xydeg(self, degrees);

        // Set values to rotated
        self.x = (x * deg.to_radians().cos()) - (y * deg.to_radians().sin()).round_dp(&DP);
        self.y = (x * deg.to_radians().sin()) + (y * deg.to_radians().cos()).round_dp(&DP);
    }

    pub fn from_rotate(vec2: &Vec2, degrees: &f32) -> Vec2 {
        /* Get a new Vec2 by rotating another Vec2 by degrees degrees to 5dp */

        // Create clones to do maths with
        let (x, y, deg) = Vec2::__clone_xydeg(vec2, degrees);

        // Return a Vec2 with rounded rotated values
        Vec2 {
            x: ((x * deg.to_radians().cos()) - (y * deg.to_radians().sin())).round_dp(&DP),
            y: ((x * deg.to_radians().sin()) + (y * deg.to_radians().cos())).round_dp(&DP),
        }
    }

    pub fn dot(&self, vec2: &Vec2) -> f32 {
        /* Returns the dot product of self and another Vec2 to 5dp */
        ((self.x * (*vec2).x) + (self.y * (*vec2).y)).round_dp(&DP)
    }

    pub fn dot_product(v1: &Vec2, v2: &Vec2) -> f32 {
        /* Returns the dot product of two Vec2s to 5dp */
        (((*v1).x * (*v2).x) + ((*v1).y * (*v2).y)).round_dp(&DP)
    }
}
