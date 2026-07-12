use raylib::prelude::*;

fn main() {

    let width = 720;
    let height = 720;

    // Crear la imagen vacia
    let mut imagen = Image::gen_image_color(width, height, Color::BLANK);

    // COLORES
    let color_pared_oscura = Color::new(105, 71, 69, 255);
    let color_naranja_claro = Color::new(235, 178, 101, 255);
    let color_naranja = Color::new(216, 110, 52, 255);
    let color_contorno = Color::new(13, 30, 46, 255);
    let color_cielo = Color::new(163, 190, 201, 255);
    let color_raya_oscura = Color::new(180, 51, 29, 255);
    let color_blanco = Color::new(255, 255, 255, 255);
    let color_marco = Color::new(62, 39, 49, 255);
    let color_crema = Color::new(244, 220, 150, 255);
    let color_pata = Color::new(161, 151, 160, 255);
    let color_ojo_azul = Color::new(30, 108, 174, 255);
    let color_mancha_roja = Color::new(45, 15, 25, 255);

    // CAPA 1 - FONDO / PARED
  
    imagen.draw_rectangle(0, 0, 720, 720, color_pared_oscura);
 

    // CAPA 2 - CUADRO EN LA PARED (marco + paisaje)
    imagen.draw_rectangle(260, 20, 420, 40, color_marco);
    imagen.draw_rectangle(240, 60, 40, 140, color_marco);
    imagen.draw_rectangle(660, 60, 40, 280, color_marco);
    imagen.draw_rectangle(260, 200, 20, 40, color_marco);
    imagen.draw_rectangle(480, 420, 20, 20, color_marco);
    imagen.draw_rectangle(500, 420, 120, 40, color_marco);
    imagen.draw_rectangle(280, 60, 380, 360, color_cielo);

    // CAPA 3 - GATO: OREJAS
    imagen.draw_rectangle(120, 200, 140, 20, color_contorno);
    imagen.draw_rectangle(320, 200, 140, 20, color_contorno);
    imagen.draw_rectangle(120, 220, 20, 40, color_contorno);
    imagen.draw_rectangle(140, 220, 20, 40, color_crema);
    imagen.draw_rectangle(160, 220, 40, 40, color_naranja_claro);
    imagen.draw_rectangle(200, 220, 40, 60, color_naranja);
    imagen.draw_rectangle(240, 220, 20, 20, color_contorno);
    imagen.draw_rectangle(320, 220, 20, 20, color_contorno);
    imagen.draw_rectangle(340, 220, 40, 40, color_naranja_claro);
    imagen.draw_rectangle(380, 220, 60, 40, color_crema);
    imagen.draw_rectangle(440, 220, 20, 40, color_contorno);
    imagen.draw_rectangle(100, 240, 20, 20, color_contorno);
    imagen.draw_rectangle(240, 240, 100, 20, color_contorno);

    // capa 4 gato, cabeza (ojos, mejillas, mancha, cuello)
    imagen.draw_rectangle(80, 260, 20, 220, color_contorno);
    imagen.draw_rectangle(100, 260, 60, 20, color_naranja_claro);
    imagen.draw_rectangle(160, 260, 40, 20, color_naranja);
    imagen.draw_rectangle(240, 260, 40, 20, color_naranja);
    imagen.draw_rectangle(280, 260, 80, 20, color_naranja_claro);
    imagen.draw_rectangle(360, 260, 60, 100, color_naranja);
    imagen.draw_rectangle(420, 260, 40, 20, color_naranja);
    imagen.draw_rectangle(460, 260, 20, 200, color_contorno);
    imagen.draw_rectangle(100, 280, 260, 60, color_naranja_claro);
    imagen.draw_rectangle(420, 280, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(420, 300, 40, 40, color_naranja);
    imagen.draw_rectangle(100, 340, 20, 40, color_blanco);
    imagen.draw_rectangle(120, 340, 60, 40, color_ojo_azul);
    imagen.draw_rectangle(180, 340, 60, 40, color_naranja_claro);
    imagen.draw_rectangle(240, 340, 60, 40, color_ojo_azul);
    imagen.draw_rectangle(300, 340, 40, 40, color_blanco);
    imagen.draw_rectangle(340, 340, 20, 40, color_naranja_claro);
    imagen.draw_rectangle(420, 340, 40, 60, color_raya_oscura);
    imagen.draw_rectangle(360, 360, 20, 40, color_naranja);
    imagen.draw_rectangle(380, 360, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(100, 380, 40, 40, color_crema);
    imagen.draw_rectangle(180, 380, 60, 40, color_crema);
    imagen.draw_rectangle(240, 380, 40, 100, color_naranja);
    imagen.draw_rectangle(280, 380, 80, 100, color_naranja_claro);
    imagen.draw_rectangle(380, 380, 20, 20, color_naranja);
    imagen.draw_rectangle(400, 380, 20, 20, color_raya_oscura);
    imagen.draw_rectangle(360, 400, 60, 60, color_naranja);
    imagen.draw_rectangle(420, 400, 40, 20, color_naranja);
    imagen.draw_rectangle(100, 420, 140, 60, color_crema);
    imagen.draw_rectangle(420, 420, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(420, 440, 40, 60, color_mancha_roja);
    imagen.draw_rectangle(480, 440, 20, 40, color_contorno);
    imagen.draw_rectangle(360, 460, 20, 20, color_naranja_claro);
    imagen.draw_rectangle(380, 460, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(460, 460, 20, 20, color_mancha_roja);
    imagen.draw_rectangle(100, 480, 100, 20, color_contorno);
    imagen.draw_rectangle(200, 480, 20, 240, color_contorno);
    imagen.draw_rectangle(220, 480, 20, 20, color_contorno);
    imagen.draw_rectangle(240, 480, 160, 20, color_raya_oscura);
    imagen.draw_rectangle(400, 480, 20, 20, color_mancha_roja);
    imagen.draw_rectangle(460, 480, 40, 40, color_raya_oscura);

    // capa 5 cola
    imagen.draw_rectangle(640, 340, 40, 20, color_contorno);
    imagen.draw_rectangle(620, 360, 20, 20, color_contorno);
    imagen.draw_rectangle(640, 360, 40, 20, color_naranja_claro);
    imagen.draw_rectangle(680, 360, 20, 20, color_contorno);
    imagen.draw_rectangle(600, 380, 20, 40, color_contorno);
    imagen.draw_rectangle(620, 380, 20, 40, color_naranja_claro);
    imagen.draw_rectangle(640, 380, 60, 60, color_naranja_claro);
    imagen.draw_rectangle(700, 380, 20, 40, color_contorno);
    imagen.draw_rectangle(620, 420, 20, 40, color_contorno);
    imagen.draw_rectangle(700, 420, 20, 140, color_naranja);
    imagen.draw_rectangle(640, 440, 20, 20, color_naranja_claro);
    imagen.draw_rectangle(660, 440, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(640, 460, 20, 80, color_contorno);
    imagen.draw_rectangle(660, 460, 20, 20, color_raya_oscura);
    imagen.draw_rectangle(680, 460, 20, 20, color_naranja_claro);
    imagen.draw_rectangle(500, 480, 20, 60, color_contorno);
    imagen.draw_rectangle(660, 480, 40, 40, color_naranja_claro);
    imagen.draw_rectangle(520, 520, 20, 40, color_contorno);
    imagen.draw_rectangle(660, 520, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(620, 540, 20, 100, color_contorno);
    imagen.draw_rectangle(640, 540, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(680, 540, 20, 60, color_naranja);
    imagen.draw_rectangle(540, 560, 20, 40, color_contorno);
    imagen.draw_rectangle(640, 560, 40, 20, color_naranja_claro);
    imagen.draw_rectangle(700, 560, 20, 80, color_contorno);
    imagen.draw_rectangle(640, 580, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(500, 600, 40, 120, color_naranja);
    imagen.draw_rectangle(540, 600, 20, 40, color_raya_oscura);
    imagen.draw_rectangle(560, 600, 20, 40, color_contorno);
    imagen.draw_rectangle(640, 600, 20, 60, color_naranja_claro);
    imagen.draw_rectangle(660, 600, 40, 20, color_raya_oscura);
    imagen.draw_rectangle(600, 620, 20, 40, color_contorno);
    imagen.draw_rectangle(660, 620, 20, 20, color_naranja_claro);
    imagen.draw_rectangle(680, 620, 20, 20, color_naranja);
    imagen.draw_rectangle(540, 640, 40, 80, color_raya_oscura);
    imagen.draw_rectangle(580, 640, 20, 40, color_contorno);
    imagen.draw_rectangle(620, 640, 20, 20, color_raya_oscura);
    imagen.draw_rectangle(660, 640, 20, 40, color_naranja);
    imagen.draw_rectangle(680, 640, 20, 60, color_contorno);
    imagen.draw_rectangle(600, 660, 40, 20, color_naranja_claro);
    imagen.draw_rectangle(640, 660, 20, 20, color_raya_oscura);
    imagen.draw_rectangle(580, 680, 20, 40, color_raya_oscura);
    imagen.draw_rectangle(600, 680, 20, 40, color_contorno);
    imagen.draw_rectangle(620, 680, 40, 20, color_naranja);
    imagen.draw_rectangle(660, 680, 20, 40, color_contorno);
    imagen.draw_rectangle(620, 700, 40, 20, color_raya_oscura);

    // capa 6 (pecho y patas)
    imagen.draw_rectangle(220, 500, 200, 40, color_mancha_roja);
    imagen.draw_rectangle(420, 500, 20, 20, color_mancha_roja);
    imagen.draw_rectangle(440, 500, 20, 60, color_raya_oscura);
    imagen.draw_rectangle(420, 520, 20, 60, color_naranja);
    imagen.draw_rectangle(460, 520, 20, 140, color_naranja);
    imagen.draw_rectangle(480, 520, 20, 20, color_raya_oscura);
    imagen.draw_rectangle(220, 540, 20, 40, color_contorno);
    imagen.draw_rectangle(240, 540, 120, 40, color_pata);
    imagen.draw_rectangle(360, 540, 40, 20, color_naranja_claro);
    imagen.draw_rectangle(400, 540, 20, 20, color_naranja);
    imagen.draw_rectangle(480, 540, 40, 20, color_naranja);
    imagen.draw_rectangle(360, 560, 20, 20, color_pata);
    imagen.draw_rectangle(380, 560, 40, 140, color_naranja_claro);
    imagen.draw_rectangle(440, 560, 20, 40, color_naranja);
    imagen.draw_rectangle(480, 560, 60, 40, color_raya_oscura);
    imagen.draw_rectangle(220, 580, 20, 120, color_naranja);
    imagen.draw_rectangle(240, 580, 20, 40, color_contorno);
    imagen.draw_rectangle(260, 580, 40, 40, color_blanco);
    imagen.draw_rectangle(300, 580, 60, 120, color_blanco);
    imagen.draw_rectangle(360, 580, 20, 20, color_blanco);
    imagen.draw_rectangle(420, 580, 20, 20, color_naranja_claro);
    imagen.draw_rectangle(360, 600, 20, 120, color_contorno);
    imagen.draw_rectangle(420, 600, 20, 80, color_naranja);
    imagen.draw_rectangle(440, 600, 20, 120, color_contorno);
    imagen.draw_rectangle(480, 600, 20, 40, color_raya_oscura);
    imagen.draw_rectangle(240, 620, 20, 60, color_naranja);
    imagen.draw_rectangle(260, 620, 20, 40, color_contorno);
    imagen.draw_rectangle(280, 620, 20, 40, color_blanco);
    imagen.draw_rectangle(480, 640, 20, 60, color_naranja);
    imagen.draw_rectangle(260, 660, 20, 20, color_naranja);
    imagen.draw_rectangle(280, 660, 20, 60, color_contorno);
    imagen.draw_rectangle(240, 680, 20, 40, color_blanco);
    imagen.draw_rectangle(220, 700, 20, 20, color_blanco);
    imagen.draw_rectangle(300, 700, 20, 20, color_contorno);
    imagen.draw_rectangle(320, 700, 40, 20, color_blanco);
    imagen.draw_rectangle(380, 700, 40, 20, color_blanco);

    // Guardar la imagen
    imagen.export_image("gato.png");

    println!("Imagen creada.");
}