use core::f32;

use super::*;

pub struct RotationMatrix3D;

impl RotationMatrix3D {
    pub fn get_x(radians: f32) -> Matrix<3, 3> {
        Matrix::new([
            [1.0, 0.0, 0.0],
            [0.0, f32::cos(radians), -f32::sin(radians)],
            [0.0, f32::sin(radians), f32::cos(radians)],
        ])
    }
    pub fn get_y(radians: f32) -> Matrix<3, 3> {
        Matrix::new([
            [f32::cos(radians), 0.0, f32::sin(radians)],
            [0.0, 1.0, 0.0],
            [-f32::sin(radians), 0.0, f32::cos(radians)],
        ])
    }
    pub fn get_z(radians: f32) -> Matrix<3, 3> {
        Matrix::new([
            [f32::cos(radians), -f32::sin(radians), 0.0],
            [f32::sin(radians), f32::cos(radians), 0.0],
            [0.0, 0.0, 1.0],
        ])
    }
}

pub struct RotationMatrix2D;

impl RotationMatrix2D {
    pub fn get(radians: f32) -> Matrix<2, 2> {
        Matrix::new([
            [f32::cos(radians), -f32::sin(radians)],
            [f32::sin(radians), f32::cos(radians)],
        ])
    }
}

pub fn get_projection_matrix(near: f32, far: f32, fov_rad: f32) -> Matrix<4, 4> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;

    let tangent: f32 = f32::tan(fov_rad);
    let top: f32 = near * tangent;
    let right: f32 = top * ASPECT_RATIO;

    Matrix::new([
        [near / right, 0.0, 0.0, 0.0],
        [0.0, near / top, 0.0, 0.0],
        [0.0, 0.0, (-(far + near)) / (far - near), -1.0],
        [0.0, 0.0, (-(2.0 * far * near)) / (far - near), 0.0],
    ])
}
