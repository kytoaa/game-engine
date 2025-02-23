use std::ops;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
    const ZERO: Vector2 = Vector2::new(0.0, 0.0);
    const UP: Vector2 = Vector2::new(0.0, 1.0);
    const DOWN: Vector2 = Vector2::new(0.0, -1.0);
    const LEFT: Vector2 = Vector2::new(-1.0, 0.0);
    const RIGHT: Vector2 = Vector2::new(1.0, 0.0);
}

impl Vector2 {
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }
    pub fn normalized(self) -> Vector2 {
        let magnitude = self.magnitude();
        Vector2 {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
    pub fn x(self, value: f32) -> Vector2 {
        Vector2 { x: value, ..self }
    }
    pub fn y(self, value: f32) -> Vector2 {
        Vector2 { y: value, ..self }
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
    const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    const UP: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    const DOWN: Vector3 = Vector3::new(0.0, -1.0, 0.0);
    const LEFT: Vector3 = Vector3::new(-1.0, 0.0, 0.0);
    const RIGHT: Vector3 = Vector3::new(1.0, 0.0, 0.0);
    const FORWARD: Vector3 = Vector3::new(0.0, 0.0, 1.0);
    const BACK: Vector3 = Vector3::new(0.0, 0.0, -1.0);
}

impl Vector3 {
    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) + (self.z * self.z)).sqrt()
    }
    pub fn normalized(self) -> Vector3 {
        let magnitude = self.magnitude();
        Vector3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }
    pub fn x(self, value: f32) -> Vector3 {
        Vector3 { x: value, ..self }
    }
    pub fn y(self, value: f32) -> Vector3 {
        Vector3 { y: value, ..self }
    }
    pub fn z(self, value: f32) -> Vector3 {
        Vector3 { z: value, ..self }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
