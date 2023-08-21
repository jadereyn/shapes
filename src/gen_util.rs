use rand::{thread_rng, Rng};
use eframe::epaint::*;
use std::f32::consts::*;

pub fn gen_random_regular_polygon(min_rad: f32, max_rad: f32, x_max: f32, y_max: f32, sides: i32) -> Vec<Pos2> {

    let rot: f32 = gen_random_f32(0.0, PI / 3.0);
    let r: f32 = gen_random_f32(min_rad, max_rad);
    let c: Pos2 = gen_random_pos2(r, x_max-r, r, y_max-r);

    (0..sides)
        .map(|i| 
            Pos2{
                x: c.x + r * ((i as f32 + rot) * 2.0 * PI / sides as f32).sin(),
                y: c.y + r * ((i as f32 + rot) * 2.0 * PI / sides as f32).cos(),
            })
        .collect()
}

pub fn gen_random_polygon(x_max: f32, y_max: f32, sides: i32) -> Vec<Pos2> {

    (0..sides)
        .map(|_| gen_random_pos2(0.0, x_max, 0.0, y_max))
        .collect()

}

pub fn gen_random_rect(x_range: f32, y_range: f32) -> Rect {
    
    let p1: Pos2 = gen_random_pos2(0.0, x_range, 0.0, y_range);
    let p2: Pos2 = gen_random_pos2(p1.x, x_range, p1.y, y_range);
    
    Rect {
        min: p1,
        max: p2,
    }
}

pub fn gen_random_pos2(x_start: f32, x_end: f32, y_start: f32, y_end: f32) -> Pos2 {
    Pos2 {
        x: gen_random_f32(x_start, x_end),
        y: gen_random_f32(y_start, y_end),
    }
}

pub fn gen_random_f32(start: f32, end: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(start..=end)
}

pub fn gen_random_rgb_color() -> Color32 {
    let mut rng = thread_rng();

    Color32::from_rgba_unmultiplied(rng.gen_range(0..=200), rng.gen_range(0..=200), rng.gen_range(0..=200), 100)
}