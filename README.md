# tokenizer_rs
A word tokenizer write purely on Rust.
It's currently have two tokenizers.

1. en - A space based tokenizer where each word is splitted by whitespace
2. th - A dictionary based tokenizer with "maximum matching" algorithm and some basic unknown word handling by minimizing a number of unknown characters until some known word(s) are found. 