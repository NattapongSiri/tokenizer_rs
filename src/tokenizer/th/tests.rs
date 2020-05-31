use crate::tokenizer::{Tokenizer};

#[test]
fn test_all_matched() {
    use std::io::{BufRead, BufReader};
    let file = BufReader::new(std::fs::File::open("data/th.txt").unwrap());
    let sources: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let concatenated = sources.iter().fold("".to_owned(), |mut acc, val| {
        acc.push_str(&val);
        acc
    });

    let tokenizer = super::Tokenizer::new("data/th.txt").unwrap();
    let tokens = tokenizer.tokenize(&concatenated);
    dbg!(&tokens);

    // at least one of the possibilities must match the original.
    assert!(
        tokens.into_iter()
                .zip(sources.iter())
                .all(|(actual, expected)| {
                    dbg!(&actual, &expected);
                    actual == expected.as_str()
                })
    );
}

#[test]
fn test_all_possible_triplet() {
    // This test case is useful to detect possible ambiguous tokenization case where multiple
    // atomic words compose into one meaningful word but it can still be considered valid
    // to treat it as separate atomic words.
    use std::io::{BufRead, BufReader};
    let file = BufReader::new(std::fs::File::open("data/th.txt").unwrap());
    let sources: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let mut kn_source = (sources.as_slice(), 3);

    use permutator::Permutation;
    
    // build all possible test cases
    let triplets = kn_source.permutation()
                                .map(|triplet| (
                                    triplet.iter()
                                            .fold("".to_owned(), |mut acc, val| {
                                                acc.push_str(&val); 
                                                acc
                                            }), 
                                    triplet));
    
    let tokenizer = super::Tokenizer::new("data/th.txt").unwrap();
    // test on every cases
    assert!(triplets.into_iter().all(|(case, expected)| {
        dbg!(&case, &expected);
        let tokens = tokenizer.tokenize(&case);
        dbg!(&tokens);
        // One of word break must match with expected
        tokens.iter().zip(expected.iter()).all(|(word, exp)| {
            dbg!(&word, &exp);
            word == exp
        })
    }));
}

#[test]
fn test_unknown_word() {
    let tokenizer = super::Tokenizer::new("data/th.txt").unwrap();
    let input = "เอากรรมกรที่เอาการเอางาน";
    dbg!(input.len());
    let tokens = tokenizer.tokenize(input);
    assert_eq!(tokens, &["เอา", "กรรมกร", "ที่", "เอาการเอางาน"]);
}

#[test]
fn test_th_en_word() {
    let tokenizer = super::Tokenizer::new("data/th.txt").unwrap();
    let tokens = tokenizer.tokenize("การบ้าน  easy มากๆ");
    assert_eq!(tokens, &["การบ้าน", "easy", "มากๆ"]);
}

#[test]
fn test_init_by_slice() {
    use std::io::{BufRead, BufReader};
    use crate::Tokenizer;
    let file = BufReader::new(std::fs::File::open("data/th.txt").unwrap());
    let sources: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let tokenizer = super::Tokenizer::from(sources.as_slice());
    assert_eq!(vec!["การบ้าน", "กรรมกร"], tokenizer.tokenize("การบ้านกรรมกร"));
}