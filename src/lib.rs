pub mod config;
pub mod error;
pub mod model; 
pub mod service;

pub use model::conversation::{Message, Role};
pub use service::gemini::GeminiClient;
