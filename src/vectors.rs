#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 {x, y}
    }
    
    fn __clone_xydeg(vec2: &Vec2, degrees: &f32) -> (f32, f32, f32) {
        /* A private method to get x, y, and degree value clones */
        (
            (*vec2).x.clone(),
            (*vec2).y.clone(),
            (*degrees).clone()
        )
    }
    
    pub fn rotate(&mut self, degrees: &f32) {
        /* Rotate the vector by degrees degrees */
        
        // Create clones to do maths with
        let (x, y, deg) = Vec2::__clone_xydeg(self, degrees);
        
        // Set values to rotated
        self.x = (x * deg.to_radians().cos()) - (y * deg.to_radians().sin());
        self.y = (x * deg.to_radians().sin()) + (y * deg.to_radians().cos());
    }
    
    pub fn from_rotate(vec2: &Vec2, degrees: &f32) -> Vec2 {
        // Create clones to do maths with
        let (x, y, deg) = Vec2::__clone_xydeg(vec2, degrees);
        
        // Return with rotated values
        Vec2 {
            x: (x * deg.to_radians().cos()) - (y * deg.to_radians().sin()),
            y: (x * deg.to_radians().sin()) + (y * deg.to_radians().cos())
        }
    }
    
    pub fn dot(&self, vec2: &Vec2) -> f32 {
        /* Returns the dot product of self and another Vec2 */
        (self.x * (*vec2).x) + (self.y * (*vec2).y)
    }
    
    pub fn dot_product(v1: &Vec2, v2: &Vec2) -> f32 {
        /* Returns the dot product of two Vec2s */
        ((*v1).x * (*v2).x) + ((*v1).y * (*v2).y)
    }
} 