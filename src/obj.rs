use raylib::prelude::Vector3;
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Obj {
    pub vertices: Vec<Vector3>,
    /// Indices de vértices con base cero, agrupados de tres en tres.
    pub indices: Vec<usize>,
}

pub fn load_obj(path: impl AsRef<Path>) -> Result<Obj, String> {
    let path = path.as_ref();
    let source = fs::read_to_string(path)
        .map_err(|error| format!("No se pudo leer {}: {error}", path.display()))?;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for (line_number, line) in source.lines().enumerate() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let values: Result<Vec<f32>, _> = parts.take(3).map(str::parse).collect();
                let values = values.map_err(|error| format!("Vertice invalido en linea {}: {error}", line_number + 1))?;
                if values.len() != 3 {
                    return Err(format!("Se esperaban tres coordenadas en linea {}", line_number + 1));
                }
                vertices.push(Vector3::new(values[0], values[1], values[2]));
            }
            Some("f") => {
                let face: Result<Vec<usize>, String> = parts.map(|item| {
                    let raw_index = item.split('/').next().unwrap_or("");
                    let obj_index: usize = raw_index.parse().map_err(|error| format!("Indice de cara invalido en linea {}: {error}", line_number + 1))?;
                    if obj_index == 0 || obj_index > vertices.len() {
                        return Err(format!("Indice {obj_index} fuera de rango en linea {}", line_number + 1));
                    }
                    Ok(obj_index - 1)
                }).collect();
                let face = face?;
                if face.len() != 3 {
                    return Err(format!("La cara de la linea {} debe estar triangulada (3 vertices)", line_number + 1));
                }
                indices.extend(face);
            }
            _ => {} // Ignorar comentarios, materiales, normales y otros registros OBJ.
        }
    }

    Ok(Obj { vertices, indices })
}
