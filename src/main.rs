use raylib::prelude::*;
mod line;

fn imprimir_vertice(v: Vector3) {
    println!("x = {}", v.x);
    println!("y = {}", v.y);
    println!("z = {}", v.z);
}

fn main() {
    let punto = Vector2::new(100.0, 200.0);
    let vertice = Vector3::new(3.0, 5.0, -2.0);

    println!("Vector2: x = {}, y = {}", punto.x, punto.y);
    imprimir_vertice(vertice);

    let (mut rl, thread) = raylib::init().size(640, 480).title("Lineas").build();
    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        line::point(&mut draw, punto, Color::WHITE);
        line::line(&mut draw, Vector2::new(80.0, 80.0), Vector2::new(560.0, 400.0), Color::GREEN);
    }
}
