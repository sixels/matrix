pub mod event;
pub mod matrix;
pub mod glyph;

pub use matrix::Matrix;

/// fixed fps rate
pub(crate) const FPS: usize = 90;
