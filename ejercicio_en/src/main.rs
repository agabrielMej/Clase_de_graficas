mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::draw_line;

use raylib::prelude::*;

fn main() {

    let mut fb = Framebuffer::new(500, 500);

    fb.set_current_color(Color::WHITE);

    draw_line(
        &mut fb,
        50,
        50,
        350,
        300,
    );

    fb.export("bresenham.png");

    println!("Imagen guardada.");
}