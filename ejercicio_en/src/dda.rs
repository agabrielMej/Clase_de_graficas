use raylib::prelude::*;

pub fn dda(
    image: &mut Image,
    start: Vector2,
    end: Vector2,
    color: Color,
) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let pasos = dx.abs().max(dy.abs());

    let x_inc = dx / pasos;
    let y_inc = dy / pasos;

    let mut x = start.x;
    let mut y = start.y;

    for _ in 0..=pasos as i32 {
        image.draw_pixel(
            x.round() as i32,
            y.round() as i32,
            color,
        );

        x += x_inc;
        y += y_inc;
    }
}