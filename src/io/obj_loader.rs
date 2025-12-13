use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::geometry::mesh::{RawMesh, VertexIndices};
use crate::math::vectors::Vector3;

#[derive(Debug)]
pub enum ObjLoadError {
    IoError(std::io::Error),
    ParseError(String),
    InvalidData(String),
}

impl From<std::io::Error> for ObjLoadError {
    fn from(err: std::io::Error) -> Self {
        ObjLoadError::IoError(err)
    }
}

pub struct ObjLoader;

impl ObjLoader {
    /// Загружает mesh из .obj файла
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<RawMesh, ObjLoadError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Self::load_from_reader(reader)
    }

    /// Загружает mesh из любого реализатора BufRead
    pub fn load_from_reader<R: BufRead>(reader: R) -> Result<RawMesh, ObjLoadError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let line = line.trim();

            // Пропускаем пустые строки и комментарии
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => Self::parse_vertex(&parts, &mut vertices, line_num)?,
                "f" => Self::parse_face(&parts, &mut indices, line_num)?,
                _ => {}
            }
        }

        Ok(RawMesh { vertices, indices })
    }

    /// Парсит вершину: "v x y z"
    fn parse_vertex(
        parts: &[&str],
        vertices: &mut Vec<Vector3>,
        line_num: usize,
    ) -> Result<(), ObjLoadError> {
        if parts.len() < 4 {
            return Err(ObjLoadError::ParseError(format!(
                "Line {}: vertex requires 3 coordinates, got {}",
                line_num,
                parts.len() - 1
            )));
        }

        let x = parts[1].parse::<f32>().map_err(|e| {
            ObjLoadError::ParseError(format!("Line {}: invalid x coordinate: {}", line_num, e))
        })?;
        let y = parts[2].parse::<f32>().map_err(|e| {
            ObjLoadError::ParseError(format!("Line {}: invalid y coordinate: {}", line_num, e))
        })?;
        let z = parts[3].parse::<f32>().map_err(|e| {
            ObjLoadError::ParseError(format!("Line {}: invalid z coordinate: {}", line_num, e))
        })?;

        vertices.push(Vector3::new(x, y, z));
        Ok(())
    }

    /// Парсит грань: "f v1 v2 v3" или "f v1/vt1 v2/vt2 v3/vt3" или "f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3"
    fn parse_face(
        parts: &[&str],
        indices: &mut Vec<VertexIndices>,
        line_num: usize,
    ) -> Result<(), ObjLoadError> {
        if parts.len() < 4 {
            return Err(ObjLoadError::ParseError(format!(
                "Line {}: face requires at least 3 vertices, got {}",
                line_num,
                parts.len() - 1
            )));
        }

        let mut vertex_indices = Vec::new();

        for part in &parts[1..] {
            // Возможные форматы: "v", "v/vt", "v/vt/vn"
            let vertex_data: Vec<&str> = part.split('/').collect();

            let vertex_index = vertex_data[0].parse::<usize>().map_err(|e| {
                ObjLoadError::ParseError(format!(
                    "Line {}: invalid vertex index '{}': {}",
                    line_num, vertex_data[0], e
                ))
            })?;

            // OBJ использует 1-based индексы, переводим в 0-based
            if vertex_index == 0 {
                return Err(ObjLoadError::ParseError(format!(
                    "Line {}: vertex index cannot be 0",
                    line_num
                )));
            }
            vertex_indices.push(vertex_index - 1);
        }

        // Преобразуем полигоны в треугольники (триангуляция)
        if vertex_indices.len() == 3 {
            // Уже треугольник
            indices.push(VertexIndices {
                indices: [vertex_indices[0], vertex_indices[1], vertex_indices[2]],
            });
        } else if vertex_indices.len() == 4 {
            // Квад -> 2 треугольника
            indices.push(VertexIndices {
                indices: [vertex_indices[0], vertex_indices[1], vertex_indices[2]],
            });
            indices.push(VertexIndices {
                indices: [vertex_indices[0], vertex_indices[2], vertex_indices[3]],
            });
        } else if vertex_indices.len() > 4 {
            // Триангуляция веером для произвольного полигона
            for i in 1..(vertex_indices.len() - 1) {
                indices.push(VertexIndices {
                    indices: [vertex_indices[0], vertex_indices[i], vertex_indices[i + 1]],
                });
            }
        }

        Ok(())
    }
}
