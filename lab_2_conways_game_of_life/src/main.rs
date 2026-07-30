mod framebuffer;
mod life;

use framebuffer::Framebuffer;
use life::GameOfLife;
use raylib::prelude::*;

fn main() {
    // resolucion logica del juego (grid pequeno = mejor rendimiento)
    let grid_width: usize = 100;
    let grid_height: usize = 100;

    // cada celula logica se dibuja como un bloque de cell_size x cell_size px reales
    let cell_size: i32 = 7;

    // tamano real de la ventana
    let window_width = grid_width as i32 * cell_size;
    let window_height = grid_height as i32 * cell_size;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::BLACK);

    let mut game = GameOfLife::new(grid_width, grid_height, cell_size);
    game.load_initial_pattern();

    // controla cada cuantos frames avanza una generacion,
    // para que la animacion no vaya demasiado rapido
    let ticks_per_generation = 6;
    let mut tick_counter = 0;

    while !rl.window_should_close() {
        // NOTA: no llamamos framebuffer.clear() aqui.
        // game.render() ya repinta cada celda (viva o muerta)
        // en cada frame, asi que no hace falta limpiar antes.
        game.render(&mut framebuffer);

        let texture = framebuffer.to_texture(&mut rl, &thread);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        d.draw_fps(10, 10);
        drop(d);

        tick_counter += 1;
        if tick_counter >= ticks_per_generation {
            game.step();
            tick_counter = 0;
        }
    }
}
