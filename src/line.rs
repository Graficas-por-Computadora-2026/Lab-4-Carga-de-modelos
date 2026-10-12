use raylib::prelude::{Color, RaylibDraw, Vector2};

/// Dibuja un píxel en la posición indicada.
pub fn point(draw: &mut impl RaylibDraw, position: Vector2, color: Color) {
    draw.draw_pixel(position.x.round() as i32, position.y.round() as i32, color);
}

/// Dibuja una línea usando el algoritmo incremental de Bresenham.
pub fn line(draw: &mut impl RaylibDraw, start: Vector2, end: Vector2, color: Color) {
    let mut x = start.x.round() as i32;
    let mut y = start.y.round() as i32;
    let x2 = end.x.round() as i32;
    let y2 = end.y.round() as i32;
    let dx = (x2 - x).abs();
    let sx = if x < x2 { 1 } else { -1 };
    let dy = -(y2 - y).abs();
    let sy = if y < y2 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        draw.draw_pixel(x, y, color);
        if x == x2 && y == y2 { break; }
        let twice_error = 2 * error;
        if twice_error >= dy { error += dy; x += sx; }
        if twice_error <= dx { error += dx; y += sy; }
    }
}

/// Dibuja el contorno de un triángulo conectando sus tres vértices.
pub fn triangle(draw: &mut impl RaylibDraw, a: Vector2, b: Vector2, c: Vector2, color: Color) {
    line(draw, a, b, color);
    line(draw, b, c, color);
    line(draw, c, a, color);
}
