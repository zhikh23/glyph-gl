use crate::geometry::face::Face;
use crate::geometry::mesh::Mesh;
use crate::geometry::vertex::Vertex;
use crate::math::vectors::UnitVector3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Mesh, ObjLoadError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Self::load_from_reader(reader)
    }

    /// Загружает mesh из любого реализатора BufRead
    pub fn load_from_reader<R: BufRead>(reader: R) -> Result<Mesh, ObjLoadError> {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

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
                "f" => Self::parse_face(&parts, &mut faces, line_num)?,
                "vn" => { /* Пока игнорируем нормали вершин */ }
                "vt" => { /* Игнорируем текстурные координаты */ }
                _ => { /* Игнорируем другие директивы */ }
            }
        }

        // Вычисляем нормали для граней
        let faces_with_normals = Self::calculate_face_normals(&faces, &vertices);

        Ok(Mesh::new(vertices, faces_with_normals))
    }

    /// Парсит вершину: "v x y z"
    fn parse_vertex(
        parts: &[&str],
        vertices: &mut Vec<Vertex>,
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

        vertices.push(Vertex::new(x, y, z));
        Ok(())
    }

    /// Парсит грань: "f v1 v2 v3" или "f v1/vt1 v2/vt2 v3/vt3" или "f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3"
    fn parse_face(
        parts: &[&str],
        faces: &mut Vec<RawFace>,
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

        /*for i in 1..parts.len() {
        let part = parts[i];*/
        for part in &parts[1..] {
            // Разбираем форматы: "v", "v/vt", "v/vt/vn"
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
            faces.push(RawFace {
                indices: [vertex_indices[0], vertex_indices[1], vertex_indices[2]],
            });
        } else if vertex_indices.len() == 4 {
            // Квад -> 2 треугольника
            faces.push(RawFace {
                indices: [vertex_indices[0], vertex_indices[1], vertex_indices[2]],
            });
            faces.push(RawFace {
                indices: [vertex_indices[0], vertex_indices[2], vertex_indices[3]],
            });
        } else {
            // Полигон с >4 вершинами - простейшая триангуляция веером
            for i in 1..(vertex_indices.len() - 1) {
                faces.push(RawFace {
                    indices: [vertex_indices[0], vertex_indices[i], vertex_indices[i + 1]],
                });
            }
        }

        Ok(())
    }

    /// Вычисляет нормали для всех граней
    fn calculate_face_normals(raw_faces: &[RawFace], vertices: &[Vertex]) -> Vec<Face> {
        let mut faces_with_normals = Vec::new();

        for raw_face in raw_faces {
            let v0 = &vertices[raw_face.indices[0]];
            let v1 = &vertices[raw_face.indices[1]];
            let v2 = &vertices[raw_face.indices[2]];

            // Вычисляем нормаль грани
            let edge1 = **v1 - **v0;
            let edge2 = **v2 - **v0;
            let normal_vector = edge1.cross(edge2).normalize();

            // Создаем UnitVector3 (безопасно, т.к. мы нормализовали)
            let normal = normal_vector.unwrap_or_else(|| UnitVector3::new_unchecked(0.0, 0.0, 1.0)); // fallback

            faces_with_normals.push(Face::new(
                raw_face.indices[0],
                raw_face.indices[1],
                raw_face.indices[2],
                normal,
            ));
        }

        faces_with_normals
    }
}

/// Временная структура для хранения граней без нормалей
#[derive(Debug)]
struct RawFace {
    indices: [usize; 3],
}
