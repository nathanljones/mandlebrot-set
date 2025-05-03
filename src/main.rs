use macroquad::prelude::*;
use std::collections::HashMap;
const SCREENSIZE: u32 = 900;
const WIDTH: u32 = 3;
const XSTART: f32 = -2.0;
const YSTART: f32 = -1.5;
const PIXEL_SCALE:u32 = SCREENSIZE/ WIDTH;

#[macroquad::main(window_conf)]
async fn main() {
    #[derive(Eq, Hash, PartialEq)]
    struct Pixel {
        x: u32,
        y: u32,
    }
    let mut points: HashMap<Pixel, Color> = HashMap::new();
    for y in 0..SCREENSIZE {
        for x in 0..SCREENSIZE {
            points.insert(Pixel { x, y }, calculate_pixel(x, y));
        }
    }

    loop {
        for y in 0..SCREENSIZE {
            for x in 0..SCREENSIZE {
                if points.contains_key(&Pixel { x, y }) {
                    draw_rectangle(x as f32, y as f32, 1.0, 1.0, points[&Pixel { x, y }]);
                }
            }
        }
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandlebrot Set".to_string(),
        window_width: SCREENSIZE as i32,
        window_height: SCREENSIZE as i32,
        ..Default::default()
    }
}
fn calculate_pixel(pixel_x: u32, pixel_y: u32) -> Color {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut xtemp: f32;
    let mut iteration = 0;
    let max_iteration = 1000;

    let x0 = XSTART + pixel_x as f32/PIXEL_SCALE as f32;
    let y0 = YSTART + pixel_y as f32/PIXEL_SCALE as f32;

    while x.powf(2f32) + y.powf(2f32) <= 4f32 && iteration < max_iteration {
        xtemp = x.powf(2f32) - y.powf(2f32) + x0;
        y = 2f32 * x * y + y0;
        x = xtemp;
        iteration += 1;
    }
    
    match iteration {
        1000 => BLACK,
        40..1000 => BLUE,
        32..40=>SKYBLUE,
        26..32=> WHITE,
        18..26 => YELLOW,
        15..18=> ORANGE,
        12..15=> RED,
        9..12 =>ORANGE,
        7..9 =>YELLOW,
        6..7=>RED,
        1..4=>BLUE,
        _ => SKYBLUE,
    }
}
