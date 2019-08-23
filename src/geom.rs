use crate::math;

/// Mesh
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh2d {
    pub vertices: Vec<math::Vector2>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh3d {
    pub vertices: Vec<math::Vector3>,
    pub indices: Vec<u32>,
}
