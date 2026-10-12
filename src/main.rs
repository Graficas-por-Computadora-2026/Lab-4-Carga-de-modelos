use raylib::prelude::*;
mod line;
mod obj;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const MARGIN: f32 = 60.0;

fn main() {
    let model = match obj::load_obj("assets/cubo_vista.obj") {
        Ok(model) => model,
        Err(error) => {
            eprintln!("Error al cargar el modelo: {error}");
            return;
        }
    };
    println!("Modelo cargado: {} vertices, {} triangulos", model.vertices.len(), model.indices.len() / 3);
    let (center_x, center_y, scale) = screen_transform(&model.vertices);
    let screenshot = std::env::args().any(|arg| arg == "--screenshot");

    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Lab 4 - Modelo OBJ").build();
    while !rl.window_should_close() {
        {
            let mut draw = rl.begin_drawing(&thread);
            draw.clear_background(Color::BLACK);
            for face in model.indices.chunks_exact(3) {
                let a = to_screen(model.vertices[face[0]], center_x, center_y, scale);
                let b = to_screen(model.vertices[face[1]], center_x, center_y, scale);
                let c = to_screen(model.vertices[face[2]], center_x, center_y, scale);
                line::triangle(&mut draw, a, b, c, Color::YELLOW);
            }
            for &vertex in &model.vertices {
                line::point(&mut draw, to_screen(vertex, center_x, center_y, scale), Color::WHITE);
            }
            if screenshot {
                draw.take_screenshot(&thread, "captura.png");
            }
        }
        if screenshot {
            break;
        }
    }
}

fn screen_transform(vertices: &[Vector3]) -> (f32, f32, f32) {
    let min_x = vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min);
    let max_x = vertices.iter().map(|v| v.x).fold(f32::NEG_INFINITY, f32::max);
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max);
    let scale_x = (WIDTH as f32 - 2.0 * MARGIN) / (max_x - min_x).max(f32::EPSILON);
    let scale_y = (HEIGHT as f32 - 2.0 * MARGIN) / (max_y - min_y).max(f32::EPSILON);
    ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0, scale_x.min(scale_y))
}

fn to_screen(vertex: Vector3, center_x: f32, center_y: f32, scale: f32) -> Vector2 {
    Vector2::new(
        WIDTH as f32 / 2.0 + (vertex.x - center_x) * scale,
        HEIGHT as f32 / 2.0 - (vertex.y - center_y) * scale,
    )
}
