mod vectors;
use vectors::Vec2;

fn main() {
    let mut my_vec2: Vec2 = Vec2::new(1.0, 1.0);

    let rotated: Vec2 = Vec2::from_rotate(&my_vec2, &90.0); 
    
    my_vec2.rotate(&90.0);
    dbg!(&my_vec2, &rotated);
}