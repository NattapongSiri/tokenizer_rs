//! tokenizer implementation, currently target for Thai language
//! 
//! It re-export two main module in root module.
//! - `en` - A space based tokenizer.
//! - `th` - A dictionary based tokenizer.
mod dict;
mod tokenizer;

pub use self::tokenizer::Tokenizer;
pub use self::tokenizer::en;
pub use self::tokenizer::th;