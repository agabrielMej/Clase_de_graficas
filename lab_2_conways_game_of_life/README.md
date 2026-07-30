# Lab 2 — Conway's Game of Life

Implementación de Conway's Game of Life en Rust usando `raylib-rs`.

## Estructura

- `src/framebuffer.rs` — Framebuffer respaldado por una `Image` de raylib.
  Expone `point(x, y)` para pintar un pixel y `get_color(x, y)` para leerlo.
- `src/life.rs` — Lógica del juego: grid con wrap-around (mundo toroidal),
  las 4 reglas de Conway, y funciones para cargar patrones clásicos
  (block, beehive, boat, tub, blinker, toad, beacon, pulsar,
  pentadecathlon, glider, LWSS, MWSS, Gosper Glider Gun).
- `src/main.rs` — Ventana de raylib y loop principal.

## Cómo correr

```bash
cargo run --release
```

- Grid lógico: 100x100 células.
- Cada célula se dibuja como un bloque de 7x7 píxeles reales
  (ventana final: 700x700).
- Presiona `ESC` o cierra la ventana para salir.

## Reglas implementadas

1. Una célula viva con menos de 2 vecinos vivos muere (underpopulation).
2. Una célula viva con 2 o 3 vecinos vivos sobrevive.
3. Una célula viva con más de 3 vecinos vivos muere (overpopulation).
4. Una célula muerta con exactamente 3 vecinos vivos nace (reproduction).

Los vecinos se cuentan con wrap-around: los bordes del grid están
conectados con el lado opuesto (mundo toroidal), lo que produce
patrones más interesantes cerca de las orillas.

## Patrón inicial

`GameOfLife::load_initial_pattern()` coloca:

- **Still lifes**: block, beehive, boat, tub
- **Osciladores**: blinker, toad, beacon, pulsar, pentadecathlon
- **Spaceships**: glider (x2), LWSS, MWSS
- **Generador infinito**: Gosper Glider Gun

## GIF

<!-- Coloca aquí el GIF mostrando el juego corriendo -->
![Demo](demo.gif)
