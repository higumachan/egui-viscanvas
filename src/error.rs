use egui::load::LoadError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum VisCanvasError {
    #[error("Load error: {0}")]
    LoadError(#[from] LoadError),
}

pub type VisCanvasResult<T> = std::result::Result<T, VisCanvasError>;

pub(crate) type Result<T> = VisCanvasResult<T>;
