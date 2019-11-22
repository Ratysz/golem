// TODO: validate vec and matrix dimensions
// TODO: add out-of-memory to GolemError?

pub mod buffer;
pub mod objects;
pub mod program;

mod context;
pub use self::context::Context;

#[derive(Debug)]
pub enum GolemError {
    /// The OpenGL Shader compilation failed, with the given error message
    ///
    /// This may be during vertex-time, fragment-time, or link-time
    ShaderCompilationError(String),
    /// Some general error bubbling up from the GL context
    ContextError(String),
}

impl From<String> for GolemError {
    fn from(other: String) -> Self {
        GolemError::ContextError(other)
    }
}
