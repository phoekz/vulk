use super::*;

pub type Vec3 = glam::Vec3;
pub type Mat4 = glam::Mat4;

pub mod clear;
pub mod compute;
pub mod cube;
pub mod triangle;

pub use clear::Demo as ClearDemo;
pub use compute::Demo as ComputeDemo;
pub use cube::Demo as CubeDemo;
pub use triangle::Demo as TriangleDemo;
