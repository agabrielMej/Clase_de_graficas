use crate::framebuffer::Framebuffer;
use raylib::prelude::Color;

pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    cells: Vec<bool>,
    next: Vec<bool>,
    // tamaño en pixeles reales de cada celula al dibujarla.
    // permite que el grid logico (ej. 100x100) se vea grande
    // en una ventana mas grande, ej 700x700.
    pub cell_size: i32,
    alive_color: Color,
    dead_color: Color,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize, cell_size: i32) -> Self {
        GameOfLife {
            width,
            height,
            cells: vec![false; width * height],
            next: vec![false; width * height],
            cell_size,
            alive_color: Color::WHITE,
            dead_color: Color::BLACK,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn set_alive(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as usize, y as usize);
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx] = true;
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cells[self.index(x, y)]
    }

    /// color logico de una celula (independiente del framebuffer)
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if self.is_alive(x, y) {
            self.alive_color
        } else {
            self.dead_color
        }
    }

    /// cuenta vecinos vivos con wrap-around (mundo toroidal),
    /// asi las celulas que "salen" de un lado aparecen del otro
    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let w = self.width as i32;
        let h = self.height as i32;
        let xi = x as i32;
        let yi = y as i32;

        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (xi + dx + w) % w;
                let ny = (yi + dy + h) % h;
                if self.cells[self.index(nx as usize, ny as usize)] {
                    count += 1;
                }
            }
        }
        count
    }

    /// aplica las 4 reglas de Conway y avanza un turno
    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let n = self.count_neighbors(x, y);

                let will_live = match (alive, n) {
                    (true, 2) | (true, 3) => true, // survival
                    (true, _) => false,            // under/overpopulation
                    (false, 3) => true,            // reproduction
                    (false, _) => false,
                };

                let idx = self.index(x, y);
                self.next[idx] = will_live;
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next);
    }

    /// dibuja el estado actual en el framebuffer. No se limpia el
    /// framebuffer antes, porque aqui se repinta cada celda
    /// (viva o muerta) en cada frame.
    pub fn render(&self, fb: &mut Framebuffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.get_color(x, y);
                fb.set_current_color(color);

                let px0 = x as i32 * self.cell_size;
                let py0 = y as i32 * self.cell_size;
                for py in py0..py0 + self.cell_size {
                    for px in px0..px0 + self.cell_size {
                        fb.point(px, py);
                    }
                }
            }
        }
    }

    // ---------- PATRONES CLASICOS ----------
    // (ox, oy) = esquina superior izquierda donde se coloca el patron

    pub fn add_block(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_beehive(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_boat(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_tub(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [(1, 0), (0, 1), (2, 1), (1, 2)] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_blinker(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [(0, 0), (1, 0), (2, 0)] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_toad(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_beacon(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [
            (0, 0), (1, 0), (0, 1), (1, 1),
            (2, 2), (3, 2), (2, 3), (3, 3),
        ] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_pulsar(&mut self, ox: i32, oy: i32) {
        let offsets = [2, 3, 4, 8, 9, 10];
        for &r in &[0i32, 5, 7, 12] {
            for &c in &offsets {
                self.set_alive(ox + c, oy + r);
            }
        }
        for &c in &[0i32, 5, 7, 12] {
            for &r in &offsets {
                self.set_alive(ox + c, oy + r);
            }
        }
    }

    pub fn add_pentadecathlon(&mut self, ox: i32, oy: i32) {
        let cells = [
            (1, 0), (1, 1),
            (0, 2), (2, 2),
            (1, 3), (1, 4), (1, 5), (1, 6),
            (0, 7), (2, 7),
            (1, 8), (1, 9),
        ];
        for (dx, dy) in cells {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_glider(&mut self, ox: i32, oy: i32) {
        for (dx, dy) in [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)] {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_lwss(&mut self, ox: i32, oy: i32) {
        let cells = [
            (1, 0), (4, 0),
            (0, 1),
            (0, 2), (4, 2),
            (0, 3), (1, 3), (2, 3), (3, 3),
        ];
        for (dx, dy) in cells {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    pub fn add_mwss(&mut self, ox: i32, oy: i32) {
        let cells = [
            (2, 0),
            (0, 1), (4, 1),
            (5, 2),
            (0, 3), (5, 3),
            (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
        ];
        for (dx, dy) in cells {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    /// bonus: Gosper Glider Gun, dispara gliders indefinidamente
    pub fn add_gosper_glider_gun(&mut self, ox: i32, oy: i32) {
        let cells = [
            (24, 0),
            (22, 1), (24, 1),
            (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
            (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
            (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
            (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
            (10, 6), (16, 6), (24, 6),
            (11, 7), (15, 7),
            (12, 8), (13, 8),
        ];
        for (dx, dy) in cells {
            self.set_alive(ox + dx, oy + dy);
        }
    }

    /// carga un patron inicial creativo que llena buena parte del grid,
    /// combinando still lifes, osciladores y spaceships
    pub fn load_initial_pattern(&mut self) {
        // still lifes
        self.add_block(2, 2);
        self.add_beehive(8, 2);
        self.add_boat(15, 2);
        self.add_tub(20, 2);

        // osciladores
        self.add_blinker(5, 15);
        self.add_toad(15, 15);
        self.add_beacon(25, 15);
        self.add_pulsar(40, 5);
        self.add_pentadecathlon(65, 10);

        // spaceships
        self.add_glider(5, 30);
        self.add_glider(50, 60);
        self.add_lwss(20, 40);
        self.add_mwss(60, 40);

        // generador infinito de gliders
        self.add_gosper_glider_gun(10, 60);
    }
}
