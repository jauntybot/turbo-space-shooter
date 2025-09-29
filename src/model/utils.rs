use super::*;

// convienent wrapper struct for rectangular hitboxes
#[turbo::serialize]
pub struct Hitbox {
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32,
}

// Function to check collision between two hitboxes
#[rustfmt::skip]
pub fn check_collision(hitbox1: &Hitbox, hitbox2: &Hitbox) -> bool {
    let x1 = hitbox1.x as i32;
    let y1 = hitbox1.y as i32;
    let w1 = hitbox1.w as i32;
    let h1 = hitbox1.h as i32;
    let x2 = hitbox2.x as i32;
    let y2 = hitbox2.y as i32;
    let w2 = hitbox2.w as i32;
    let h2 = hitbox2.h as i32;
    x1 < x2 + w2 && x1 + w1 > x2 &&
    y1 < y2 + h2 && y1 + h1 > y2
}

// Pseudo-random number generator
pub fn rand_with_seed(seed: u32) -> u32 {
    (seed * 1103515245 + 12345) % 2147483648
}
