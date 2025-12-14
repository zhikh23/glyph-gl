use crate::geometry::aabb::Aabb;
use crate::math::vectors::{Normal3, UnitVector3, Vector3};

pub struct RawMesh {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<VertexIndices>,
}

pub struct VertexIndices {
    pub indices: [usize; 3],
}

pub struct TriangleRef {
    vertex_indices: [usize; 3],
    normal_indices: [usize; 3],
}

pub struct Mesh {
    vertices: Vec<Vector3>,
    normals: Vec<Normal3>,
    triangles: Vec<TriangleRef>,
}

#[derive(thiserror::Error, Debug)]
pub enum MeshError {
    #[error("vertex index out of range: {0}")]
    VertexIndexOutOfRange(usize),

    #[error("normal index out of range: {0}")]
    NormalIndexOutOfRange(usize),

    #[error("degenerate face: {0}")]
    DegenerateTriangle(usize),
}

impl Mesh {
    pub fn with_flat_normals(raw: RawMesh) -> Result<Mesh, MeshError> {
        let mut normals = Vec::with_capacity(raw.indices.len()); // одна нормаль на грань
        let mut raw_faces = Vec::with_capacity(raw.indices.len());
        for tr in raw.indices.iter() {
            let (v0, v1, v2) = (
                raw.vertices[tr.indices[0]],
                raw.vertices[tr.indices[1]],
                raw.vertices[tr.indices[2]],
            );
            let normal = Self::compute_face_normal(v0, v1, v2)
                //.ok_or(MeshError::DegenerateTriangle(i))?;
                .unwrap_or(Normal3::new_unchecked(0.0, 0.0, 1.0));
            normals.push(normal);
            let n_index = normals.len() - 1;
            raw_faces.push(TriangleRef {
                vertex_indices: tr.indices,
                normal_indices: [n_index, n_index, n_index],
            });
        }
        Self::check_indices(&raw.vertices, &normals, &raw_faces)?;
        Ok(Self::new_unchecked(raw.vertices, normals, raw_faces))
    }

    pub fn with_smooth_normals(raw: RawMesh) -> Result<Mesh, MeshError> {
        for triangle in raw.indices.iter() {
            for &idx in triangle.indices.iter() {
                if idx >= raw.vertices.len() {
                    return Err(MeshError::VertexIndexOutOfRange(idx));
                }
            }
        }

        let mut accumulated_normals = vec![Vector3::zero(); raw.vertices.len()];
        for tr in raw.indices.iter() {
            let v0 = raw.vertices[tr.indices[0]];
            let v1 = raw.vertices[tr.indices[1]];
            let v2 = raw.vertices[tr.indices[2]];
            let face_normal = Self::compute_face_normal(v0, v1, v2)
                //.ok_or(MeshError::DegenerateTriangle(i))?;
                .unwrap_or(Normal3::new_unchecked(0.0, 0.0, 1.0));

            for &vertex_idx in tr.indices.iter() {
                accumulated_normals[vertex_idx] += face_normal.downgrade();
            }
        }

        let mut vertex_normals = Vec::with_capacity(raw.vertices.len());
        for acc_nor in accumulated_normals {
            let normal = acc_nor
                .normalize()
                .unwrap_or(UnitVector3::new_unchecked(0.0, 1.0, 0.0));
            vertex_normals.push(normal);
        }

        let mut mesh_triangles = Vec::with_capacity(raw.indices.len());
        for triangle in raw.indices {
            mesh_triangles.push(TriangleRef {
                vertex_indices: triangle.indices,
                normal_indices: triangle.indices, // Каждая вершина имеет соответствующую нормаль
            });
        }

        Ok(Self {
            vertices: raw.vertices,
            normals: vertex_normals,
            triangles: mesh_triangles,
        })
    }

    fn compute_face_normal(v0: Vector3, v1: Vector3, v2: Vector3) -> Option<Normal3> {
        let v1v0 = v1 - v0;
        let v2v0 = v2 - v0;
        v1v0.cross(v2v0).normalize()
    }

    fn check_indices(
        vertices: &[Vector3],
        normals: &[Normal3],
        triangles: &[TriangleRef],
    ) -> Result<(), MeshError> {
        for rf in triangles.iter() {
            for &v_idx in rf.vertex_indices.iter() {
                if v_idx >= vertices.len() {}
            }
            for &n_idx in rf.normal_indices.iter() {
                if n_idx >= normals.len() {
                    return Err(MeshError::NormalIndexOutOfRange(n_idx));
                }
            }
        }
        Ok(())
    }

    fn new_unchecked(
        vertices: Vec<Vector3>,
        normals: Vec<Normal3>,
        triangles: Vec<TriangleRef>,
    ) -> Mesh {
        Mesh {
            vertices,
            normals,
            triangles,
        }
    }

    pub fn triangle(&self, idx: usize) -> Option<Triangle> {
        let tr = self.triangles.get(idx)?;
        let vertices = std::array::from_fn(|i| Vertex {
            pos: self.vertices[tr.vertex_indices[i]],
            nor: self.normals[tr.normal_indices[i]],
        });
        Some(Triangle { vertices })
    }

    pub fn iter(&self) -> MeshIterator<'_> {
        MeshIterator { mesh: self, idx: 0 }
    }

    pub fn center(&self) -> Vector3 {
        let acc = self
            .vertices
            .iter()
            .fold(Vector3::zero(), |acc, v| acc + *v);
        acc * (1.0 / self.vertices.len() as f32)
    }

    pub fn scale(&mut self, scale: f32) {
        self.vertices.iter_mut().for_each(|v| *v = *v * scale);
    }

    pub fn fit(&mut self, max_extent: f32) {
        let aabb = self.aabb();
        self.scale(max_extent / aabb.max_extent());
    }

    pub fn aabb(&self) -> Aabb {
        let left = self
            .vertices
            .iter()
            .fold(f32::INFINITY, |acc, v| acc.min(v.x));
        let right = self
            .vertices
            .iter()
            .fold(f32::NEG_INFINITY, |acc, v| acc.max(v.x));
        let bottom = self
            .vertices
            .iter()
            .fold(f32::INFINITY, |acc, v| acc.min(v.y));
        let top = self
            .vertices
            .iter()
            .fold(f32::NEG_INFINITY, |acc, v| acc.max(v.y));
        let far = self
            .vertices
            .iter()
            .fold(f32::INFINITY, |acc, v| acc.min(v.z));
        let near = self
            .vertices
            .iter()
            .fold(f32::NEG_INFINITY, |acc, v| acc.max(v.z));
        Aabb::new(left, right, bottom, top, far, near)
    }
}

#[derive(Clone)]
pub struct Vertex {
    pub pos: Vector3,
    pub nor: Normal3,
}

pub struct Triangle {
    vertices: [Vertex; 3],
}

impl Triangle {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
}

pub struct MeshIterator<'a> {
    mesh: &'a Mesh,
    idx: usize,
}

impl Iterator for MeshIterator<'_> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx + 1 < self.mesh.triangles.len() {
            self.idx += 1;
            self.mesh.triangle(self.idx)
        } else {
            None
        }
    }
}
