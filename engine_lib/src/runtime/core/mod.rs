use matrices::Matrix;

pub use crate::core::datatypes::*;

pub struct Transform {
    position: vectors::Vector3,
    rotation: vectors::Vector3,
    scale: vectors::Vector3,
}

impl Transform {
    pub fn transformation_matrix(&self) -> Matrix<3, 3> {
        let matrix = Matrix::new([
            [self.scale.x, 0.0, 0.0],
            [0.0, self.scale.y, 0.0],
            [0.0, 0.0, self.scale.z],
        ]);
        let matrix = matrix
            * matrices::RotationMatrix3D::get_z(self.rotation.z)
            * matrices::RotationMatrix3D::get_y(self.rotation.y)
            * matrices::RotationMatrix3D::get_x(self.rotation.x);
        matrix
    }
}
