//! tokenizer implementation, currently target for Thai language
//! 
//! It re-export two main module in root module.
//! - `en` - A space based tokenizer.
//! - `th` - A dictionary based tokenizer.
mod dict;
mod tokenizer;

pub use tokenizer::Tokenizer;
pub use tokenizer::en;
pub use tokenizer::th;