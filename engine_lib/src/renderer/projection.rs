use super::RenderVert;
use crate::core::datatypes::matrices;
use crate::core::datatypes::meshes::Vert;

pub enum Projection {
    Orthographic,
    Perspective { near: f32, far: f32, fov: f32 },
}

pub fn project(
    projection: Projection,
    verts: Box<dyn Iterator<Item = Vert>>,
) -> impl Iterator<Item = RenderVert> {
    match projection {
        Projection::Orthographic => todo!(),
        Projection::Perspective { near, far, fov } => perspective(verts, near, far, fov),
    }
    .map(|matrix| {
        println!(
            "{}, {}, {}, {}",
            matrix.values[0][0], matrix.values[1][0], matrix.values[2][0], matrix.values[3][0]
        );
        normalize_to_screen(matrix)
        //RenderVert::new([matrix.get((0, 0)).unwrap(), matrix.get((1, 0)).unwrap()])
    })
}

fn perspective(
    verts: Box<dyn Iterator<Item = Vert>>,
    near: f32,
    far: f32,
    fov: f32,
) -> impl Iterator<Item = matrices::Matrix<4, 1>> {
    let projection_matrix = matrices::get_projection_matrix(near, far, fov);
    Box::new(verts.map(move |vert| {
        let matrix = matrices::Matrix::new([
            [vert.position.x],
            [vert.position.y],
            [vert.position.z],
            [1.0],
        ]);
        projection_matrix.matrix_multiply(matrix)
    }))
}

fn normalize_to_screen(matrix: matrices::Matrix<4, 1>) -> RenderVert {
    let (x, y, _z, w) = (
        matrix.get((0, 0)).unwrap(),
        matrix.get((1, 0)).unwrap(),
        matrix.get((2, 0)).unwrap(),
        matrix.get((3, 0)).unwrap(),
    );

    if w == 0.0 {
        return RenderVert::new([0.0, 0.0]);
    }
    let px = (x / w + 1.0) / 2.0;
    let py = (y / w + 1.0) / 2.0;

    println!("position: {}, {}", px, py);

    RenderVert::new([px, py])
}
