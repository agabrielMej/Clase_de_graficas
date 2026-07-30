use raylib::prelude::*;

/// Framebuffer sencillo respaldado por una Image de raylib.
/// La lógica de dibujo (point) trabaja sobre esta imagen en CPU;
/// cada frame la convertimos a una Texture2D para poder dibujarla
/// en la ventana con raylib.
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    /// Regresa el framebuffer a un solo color (background_color).
    /// Para Conway's Game of Life NO se llama esto en el loop principal,
    /// porque render() ya repinta cada celda (viva o muerta) en cada
    /// frame, así que no se necesita limpiar antes de volver a dibujar.
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Pinta un pixel individual en (x, y) usando el current_color.
    /// Esta es la función base que usa todo lo demás (líneas, polígonos,
    /// y ahora Game of Life).
    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    /// Regresa el color actual de un pixel del framebuffer.
    /// NOTA: Image::get_color de raylib-rs pide &mut self internamente,
    /// aunque solo lea el pixel, así que este método también es &mut.
    pub fn get_color(&mut self, x: i32, y: i32) -> Color {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.get_color(x, y)
        } else {
            self.background_color
        }
    }

    /// Convierte el framebuffer actual en una textura de GPU lista
    /// para dibujarse en pantalla con d.draw_texture(...).
    pub fn to_texture(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
        rl.load_texture_from_image(thread, &self.color_buffer)
            .expect("No se pudo crear la textura a partir del framebuffer")
    }

    /// Exporta el framebuffer actual como imagen (útil para debug/capturas).
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
