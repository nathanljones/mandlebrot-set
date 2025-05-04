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

    if iteration == max_iteration {
        return BLACK;
    }
    if iteration == 0 {
        return BLACK
    }
    match iteration % 16 {
        0 => Color::from_rgba(66, 30, 15,255),
        1 => Color::from_rgba(25, 7, 26,255),
        2 => Color::from_rgba(9, 1, 47,255),
        3 => Color::from_rgba(4, 4, 73,255),
        4 => Color::from_rgba(0, 7, 100,255),
        5 => Color::from_rgba(12, 44, 138,255),
        6 => Color::from_rgba(24, 82, 177,255),
        7 => Color::from_rgba(57, 125, 209,255),
        8 => Color::from_rgba(134, 181, 229,255),
        9 => Color::from_rgba(211, 236, 248,255),
        10 => Color::from_rgba(241, 233, 191,255),
        11 => Color::from_rgba(248, 201, 95,255),
        12 => Color::from_rgba(255, 170, 0,255),
        13 => Color::from_rgba(204, 128, 0,255),
        14 => Color::from_rgba(153, 87, 0,255),
        15 => Color::from_rgba(106, 52, 3,255),
        _ => BLACK,
    }
}
