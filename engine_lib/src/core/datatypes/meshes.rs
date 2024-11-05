use super::vectors::Vector3;

pub struct Vert {
    pub position: Vector3,
}

pub struct Tri {
    pub verts: [Vert; 3],
}

pub struct Mesh {
    pub tris: Vec<Tri>,
}
