/// borrowed from glam-rs, cgmath

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn dim() -> i32 {
        2
    }
}

#[inline]
pub fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2 { x: x, y: y }
}

impl From<[f32; 2]> for Vector2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }
}

impl From<Vector2> for [f32; 2] {
    #[inline]
    fn from(v: Vector2) -> Self {
        [v.x, v.y]
    }
}

impl From<Vector2> for Vec<f32> {
    #[inline]
    fn from(v: Vector2) -> Self {
        vec![v.x, v.y]
    }
}

/// A 3-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn dim() -> i32 {
        3
    }
}

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x: x, y: y, z: z }
}

impl From<[f32; 3]> for Vector3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
}

impl From<Vector3> for [f32; 3] {
    #[inline]
    fn from(v: Vector3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<Vector3> for Vec<f32> {
    #[inline]
    fn from(v: Vector3) -> Self {
        vec![v.x, v.y, v.z]
    }
}
