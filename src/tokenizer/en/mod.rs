/// White space based tokenizer. It split word based on white space.
struct Tokenizer;

// /// Common acronym. This should be useful for sentence tokenizer.
// const acronym: &'static [&'static str] = &[
//     "Mr.", "Mrs.", "Doc.", "Prof.", // People honorific
//     "Mon.", "Tue.", "Wed.", "Thu.", "Fri.", "Sat.", "Sun.", // Three chars date
//     "Jan.", "Feb.", "Mar.", "Aprl.", "Jun.", "Sep.", "Aug.", "Oct.", "Nov.", "Dec.", // 3-4 Chars month
// ];

impl super::Tokenizer for Tokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        text.split_whitespace().collect()
    }
}