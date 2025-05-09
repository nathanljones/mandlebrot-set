use itertools::Itertools;
use macroquad::prelude::*;
use rayon::prelude::*;

const SCREENSIZE: u32 = 900; // Pixel size of screen (will be square so only need one value
const WIDTH: u32 = 3; // Width of the graph (3 units)
const XSTART: f32 = -2.0; // Graph starts at -2x
const YSTART: f32 = -1.5; // Graph ends at -1.5
const PIXEL_SCALE: u32 = SCREENSIZE / WIDTH; // Scale the graph to the screen

struct Pixel {
    x: u32,
    y: u32,
}

struct Point {
    pixel: Pixel,
    colour: Color,
}

#[macroquad::main(window_conf)]
async fn main() {
    let points: Vec<Point> = (0..SCREENSIZE)
        .cartesian_product(0..SCREENSIZE)
        .par_bridge()
        .map(|(x, y)| Point {
            pixel: Pixel { x, y },
            colour: calculate_pixel(x, y),
        })
        .collect();

    loop {
        points.iter().for_each(|point| {
            draw_rectangle(
                point.pixel.x as f32,
                point.pixel.y as f32,
                1.0,
                1.0,
                point.colour,
            );
        });
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

    // Convert the pixel value to the graph value
    let x0 = XSTART + pixel_x as f32 / PIXEL_SCALE as f32;
    let y0 = YSTART + pixel_y as f32 / PIXEL_SCALE as f32;

    // Main calculation for a point on the Mandlebrot set
    while x.powf(2f32) + y.powf(2f32) <= 4f32 && iteration < max_iteration {
        xtemp = x.powf(2f32) - y.powf(2f32) + x0;
        y = 2f32 * x * y + y0;
        x = xtemp;
        iteration += 1;
    }

    // the next section just converts the number of iterations into a colour to make
    // it look pretty
    if iteration == max_iteration {
        return BLACK;
    }
    if iteration == 0 {
        return BLACK;
    }
    match iteration % 16 {
        0 => Color::from_rgba(66, 30, 15, 255),
        1 => Color::from_rgba(25, 7, 26, 255),
        2 => Color::from_rgba(9, 1, 47, 255),
        3 => Color::from_rgba(4, 4, 73, 255),
        4 => Color::from_rgba(0, 7, 100, 255),
        5 => Color::from_rgba(12, 44, 138, 255),
        6 => Color::from_rgba(24, 82, 177, 255),
        7 => Color::from_rgba(57, 125, 209, 255),
        8 => Color::from_rgba(134, 181, 229, 255),
        9 => Color::from_rgba(211, 236, 248, 255),
        10 => Color::from_rgba(241, 233, 191, 255),
        11 => Color::from_rgba(248, 201, 95, 255),
        12 => Color::from_rgba(255, 170, 0, 255),
        13 => Color::from_rgba(204, 128, 0, 255),
        14 => Color::from_rgba(153, 87, 0, 255),
        15 => Color::from_rgba(106, 52, 3, 255),
        _ => BLACK,
    }
}
