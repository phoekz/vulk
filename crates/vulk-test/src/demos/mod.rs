use super::*;

pub type Vec2 = glam::Vec2;
pub type Vec3 = glam::Vec3;

pub mod clear;
pub mod compute;
pub mod triangle;

pub use clear::Demo as ClearDemo;
pub use compute::Demo as ComputeDemo;
pub use triangle::Demo as TriangleDemo;
