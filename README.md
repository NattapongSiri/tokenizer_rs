# tokenizer_rs
A word tokenizer write purely on Rust.
It's currently have two tokenizers.
1. en - A space based tokenizer where each word is splitted by whitespace
1. th - A dictionary based tokenizer with "maximum matching" algorithm and some basic unknown word handling by minimizing a number of unknown characters until some known word(s) are found. 

It currently support two feature gate:
- `multi-thread` - It will attempt to use multi-thread for tokenization.
- `single-thread` - It will use single thread.

As currently is, Thai word tokenizer support both features. It use [Rayon](https://crates.io/crates/rayon) to do multi-thread tokenization. It simply split text by white space first then on each chunk, attempt tokenization on each chunk on separate thread using `Rayon` parallel iterator.

English language doesn't actually leverage multi-thread yet but it will work on both feature.

By default, it will use `multi-thread`

# How to use
Put following line in your `cargo.toml` dependencies section.
For example:
```toml
[dependencies]
tokenizer = "^0.1"
```
It will attempt to use multi-thread to do tokenization.

To force single-thread, use `single-thread` feature.
```toml
[dependencies]
tokenizer = { version = "^0.1", features = ["single-thread"] }
```

An example of Thai text tokenization:
```rust
use tokenizer::{Tokenizer, th};
let tokenizer = th::Tokenizer::new("path/to/dictionary.txt").expect("Dictionary file not found");
// Assuming dictinoary contains "ภาษาไทย" and "นิดเดียว" but not "ง่าย"
assert_eq!(tokenizer.tokenize("ภาษาไทยง่ายนิดเดียว"), vec!["ภาษาไทย", "ง่าย", "นิดเดียว"]);
```