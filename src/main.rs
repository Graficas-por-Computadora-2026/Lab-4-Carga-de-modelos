use raylib::prelude::*;
mod line;
mod obj;

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

    let model = match obj::load_obj("assets/cubo_triangulado.obj") {
        Ok(model) => model,
        Err(error) => {
            eprintln!("Error al cargar el modelo: {error}");
            return;
        }
    };
    println!("Modelo cargado: {} vertices, {} triangulos", model.vertices.len(), model.indices.len() / 3);

    let (mut rl, thread) = raylib::init().size(640, 480).title("Cargador OBJ").build();
    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        for face in model.indices.chunks_exact(3) {
            let a = project(model.vertices[face[0]]);
            let b = project(model.vertices[face[1]]);
            let c = project(model.vertices[face[2]]);
            line::triangle(&mut draw, a, b, c, Color::YELLOW);
        }
        for &vertex in &model.vertices {
            line::point(&mut draw, project(vertex), Color::WHITE);
        }
    }
}

fn project(vertex: Vector3) -> Vector2 {
    Vector2::new(
        320.0 + (vertex.x - vertex.z) * 90.0,
        240.0 + (vertex.x + vertex.z) * 35.0 - vertex.y * 90.0,
    )
}
