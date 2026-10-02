use raylib::prelude::*;

fn imprimir_vertice(v: Vector3) {
    println!("x = {}", v.x);
    println!("y = {}", v.y);
    println!("z = {}", v.z);
}

fn main() {
    let punto = Vector2::new(100.0, 200.0);
    let vertice = Vector3::new(3.0, 5.0, -2.0);

    imprimir_vertice(vertice);
}
