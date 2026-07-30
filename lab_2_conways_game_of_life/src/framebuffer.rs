use raylib::prelude::*;

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

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn get_color(&mut self, x: i32, y: i32) -> Color {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.get_color(x, y)
        } else {
            self.background_color
        }
    }

    pub fn to_texture(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
        rl.load_texture_from_image(thread, &self.color_buffer)
            .expect("No se pudo crear la textura a partir del framebuffer")
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
