//! Dictionary based Thai word tokenizer
//! 
//! It uses Dictionary to lookup for a known word. If there's multiple
//! possible ways to tokenize words, it choose a result that has least number of words.
//! This is known as "maximum matching" algorithm. It is one of the most common use
//! algorithm that produce acceptable quality.
//! 
//! It can handle some unknown words. It does so by minimizing number of characters 
//! that need to be took off from the text until a known word is found. 

use crate::dict::{SizedNode, terminals_prefix};
use super::MultiOwn;
use super::{TreeOp, TreeNode};

/// Extra metadata required to get a proper tokenization on Thai text.
#[allow(dead_code)]
struct LeafNode<T> {
    /// An actual leaf node on the tree.
    node: T,
    /// A bytes count of unknown characters on the branch of tokenization tree
    unknown_bytes_count: usize
}

/// Make a result tree contains all possible construct of `unit` combination.
/// Caller typicall need to do `make_result_tree(&*dict.root, "SOME_TEXT_TO_PARSE", root_node)`
/// 
/// Typically, `unit` is a word.
/// 
/// # Parameters
/// - `nodes` - Set of root dictionary nodes. It's typical in [SizedDict](struct.SizedDict.html#field.root)
/// - `value` - A string to be parsed by given dictionary.
/// - `parent` - A [TreeNode](struct.TreeNode.html) that will be root node of all the parsed unit
/// - `leaves` - A Vec contains all the possible leaves nodes.
#[allow(dead_code)]
fn make_result_tree<'a>(nodes: &[SizedNode], value: &'a str, parent: MultiOwn<TreeNode<&'a str>>, leaves: &mut Vec<LeafNode<MultiOwn<TreeNode<&'a str>>>>) {
    #[cfg(not(feature="single-thread"))]
    fn add_child<'a>(parent: &MultiOwn<TreeNode<&'a str>>, value: &'a str, upto: usize) -> MultiOwn<TreeNode<&'a str>> {
        std::sync::Arc::clone(parent).add_child(&value[..upto])
    }
    #[cfg(feature="single-thread")]
    fn add_child<'a>(parent: &MultiOwn<TreeNode<&'a str>>, value: &'a str, upto: usize) -> MultiOwn<TreeNode<&'a str>> {
        std::rc::Rc::clone(parent).add_child(&value[..upto])
    }
    /// Consume a portion of value until either entire value is consumed or token(s) are found.
    /// 
    /// If it consumed entire value, it will add a leaf node into leaves vec.
    /// 
    /// If it consumed chunk of value, it will add a node to parent node, change pointer to parent node to new node,
    /// change pointer to value to point to remaining slice, and add all potential known tokens to result vec.
    /// 
    /// In anycase, it will update consumed_bytes but not accumulated_unknown_bytes.
    #[inline(always)]
    fn consume_unknown<'a>(nodes: &[SizedNode], value: &mut &'a str, accumulated_unknown_bytes: usize, consumed_bytes: &mut usize, parent: &mut MultiOwn<TreeNode<&'a str>>, results: &mut Vec<usize>, leaves: &mut Vec<LeafNode<MultiOwn<TreeNode<&'a str>>>>) {
        // Apply some algorithm to extract unknown word and repeatly re-evaluate if the remain
        // from algorithm is a known word
        let mut chars = value.chars(); // Take a chars iterator and consume all repeating chars

        while let Some(c) = chars.next() {
            *consumed_bytes += c.len_utf8();

            terminals_prefix(nodes, value, *consumed_bytes, results);
            
            if results.len() > 0 {
                // Found an offset where sub-sequence chars is a known word.
                // Make a new node and make it parent of sun-sequence of valid words.
                *parent = add_child(parent, value, *consumed_bytes);

                // Shift value by consumed byte as the unknown word boundary is at consumed bytes.
                *value = &value[*consumed_bytes..];

                // Since the value to evaluate is a slice where start is shifted by consumed bytes,
                // the results vec need to be subtracted by consumed bytes.
                results.iter_mut().for_each(|val| *val -= *consumed_bytes);

                // stop lookup as known word is found.
                break;
            }
        }

        if results.len() == 0 {
            // Entire value is consumed and no known word found.
            // It mean there's unknown word trailing or entire value is an unknown word.
            // No need to lookup more. Simply make entire value as a single leaf node.
            let node = add_child(parent, value, *consumed_bytes);
            leaves.push(LeafNode {
                node,
                unknown_bytes_count: accumulated_unknown_bytes + *consumed_bytes,
            });
        }
    }

    let mut eval_queue = std::collections::VecDeque::new();
    eval_queue.push_back((value, parent, 0)); // 0 is accumulated unknown bytes of given tree branch
    let mut results = Vec::new();

    while let Some((mut value, mut parent, accumulated_unknown_bytes)) = eval_queue.pop_front() {
        // Each branch shall have it own remain value to be evaluate, node parent, and accumulated unknown bytes.
        results.clear();
        terminals_prefix(nodes, value, 0, &mut results);

        // A prefix bytes that was unknown word which need to be consumed in order to reach known word.
        let mut consumed_bytes = 0;

        // no match for any entry in dictionary
        if results.len() == 0 && value.len() > 0 {
            consume_unknown(nodes, &mut value, accumulated_unknown_bytes, &mut consumed_bytes, &mut parent, &mut results, leaves);
        }

        let new_accumulated_unknown_bytes = accumulated_unknown_bytes + consumed_bytes;
        
        // On each result, it is a known word offset
        results.iter().for_each(|offset| {
            let node = add_child(&parent, value, *offset);
            let remain = &value[(*offset)..];

            if remain.len() > 0 {
                // more value remain to try
                eval_queue.push_back((remain, node, new_accumulated_unknown_bytes));
            } else {
                // no more value, add matched terminal to `leaves` vec
                leaves.push(LeafNode {node, unknown_bytes_count: new_accumulated_unknown_bytes});
            }
        });
    }
}

/// Maximal matching algorithm with unknown word support.
/// 
/// This is an implementation based on concept of maximum matching.
/// See this [Wikipedia page](https://en.wikipedia.org/wiki/Matching_(graph_theory)#Maximal_matchings) 
/// for brief explanation of the algorithm.
/// 
/// It take a dictionary in form of &[SizedNode] and a text to be tokenized.
/// # Parameters
/// - `dict` - A slice of [dict::SizedNode](/tokenizer/dict/struct.SizedNode.html) which
/// can be obtain from `root` field of [dict::SizedDict](/tokenizer/dict/struct.SizedDict.html).
/// - `text` - A slice of string to be tokenized.
/// # Return
/// A vec contains slice of tokenized word.
fn maximal_matching<'a>(dict: &[SizedNode], text: &'a str) -> Vec<&'a str> {
    /// There's three possible states in one vertex
    #[derive(Clone)]
    enum VertexState {
        /// A vertex that nobody visit yet
        None,
        /// A vertex that is recognized in advance while attempting to isloate unknown word
        Cache(Vec<usize>),
        /// A vertex that is visited by breadth-first-search strategy
        Some(Vec<usize>)
    }

    impl Default for VertexState {
        /// By default, `VertexState` is `None`
        fn default() -> VertexState {
            VertexState::None
        }
    }

    impl VertexState {
        /// Take current value out of this `VertexState` and leave `None` in place.
        /// If current state is `None`, it return `Option::None`
        pub fn take(&mut self) -> Option<Vec<usize>> {
            let value = std::mem::take(self);
            match value {
                VertexState::None => None,
                VertexState::Cache(v) | VertexState::Some(v) => {
                    Some(v)
                },
            }
        }
    }

    /// Take as least necessary bytes as needed until first known word is found.
    /// It need a dictionary to identify a known word. It will increment an offset by one character
    /// until there's a prefix match to a dictionary entry.
    /// For example, if text is "abcdef" and "ab" is unknown word and "cd" and "cde" is known word in dic
    /// then this function will put 3, and 4 in `results` vec and return 2 which is unknown word boundary.
    /// 
    /// # Parameters
    /// - `nodes` - A slice of [dict::SizedNode](/tokenizer/dict/struct.SizedNode.html) which
    /// can be obtain from `root` field of [dict::SizedDict](/tokenizer/dict/struct.SizedDict.html).
    /// - `value` - A string slice to find an offset of unknown words.
    /// - `offset` - A position to start isolate an unknown word.
    /// - `results` - A vec which will store known words that come after the isolated unknown word.
    /// # Return
    /// It return an offset boundary of unknown word.
    /// For example, if text is "abcdef" and "cd" is the only unknown word and offset is 2,
    /// it will return 4. Caller can directly took slice from `&value[2..4]` to obtains that "cd"
    fn consume_unknown<'a>(nodes: &[SizedNode], value: &str, offset: usize, results: &mut Vec<usize>) -> usize {
        // Apply some algorithm to extract unknown word and repeatly re-evaluate if the remain
        // from algorithm is a known word
        let mut consumed_bytes = offset;
        let mut chars = value[consumed_bytes..].chars(); // Take a chars iterator and consume all repeating chars

        while let Some(c) = chars.next() {
            consumed_bytes += c.len_utf8();

            terminals_prefix(nodes, value, consumed_bytes, results);
            
            if results.len() > 0 {
                // stop lookup as known word is found.
                break;
            }
        }

        consumed_bytes
    }

    // 2D dynamic vec to simulate word graph
    let mut vertices = vec![VertexState::None; text.len()];
    // `branches` is reusable vec to temporarily hold possible branch from any particular vertex.
    let mut branches = Vec::with_capacity(text.len());
    // `queue` is a processing queue of vertex to be processed.
    // The vertex being pushed to this queue shall make a graph traversal "breadth-first"
    let mut queue = Vec::with_capacity(text.len() / 2);

    queue.push([0, 0, 0]); // previous pointer, current vertex index, unknown bytes count

    // Assuming text has at least 2 chars per word
    let mut result = std::collections::VecDeque::with_capacity(text.len() / 2); 

    let mut i = 0;
    let mut not_ended = true;

    while not_ended {
        // Retreive next offset
        let [_, offset, unknown_len] = queue[i];
        branches.clear();

        match vertices[offset] {
            VertexState::None => {
                // Find next prefix from offset
                terminals_prefix(dict, text, offset, &mut branches);

                // create state of vertex which is Vec that contains offset to vertex
                let mut vertex = Vec::with_capacity(branches.len());

                for v in branches.iter() {
                    vertex.push(*v); // add next offset to vertex state
                    queue.push([i, *v, unknown_len]);

                    if *v >= text.len() {
                        not_ended = false;
                        break;
                    }
                }

                if branches.len() > 0 {
                    // known word case
                    vertices[offset] = VertexState::Some(vertex);
                } else {
                    // No known word match so not even single branch is return
                    let cur_unknown_length = consume_unknown(dict, text, offset, &mut branches);
                    queue.push([i, cur_unknown_length, unknown_len + cur_unknown_length - offset]); // Identified unknown word boundary
                    
                    vertices[offset] = VertexState::Some(vec![cur_unknown_length]);

                    if cur_unknown_length >= text.len() { // Unknown word is trailing in text
                        break; // No need to do any futher processing
                    }

                    // All the returned branches are 1 step ahead of other branch so don't push it to queue yet
                    // or it will break breadth-first strategy
                    let mut peeked = branches.clone(); 
                    peeked.shrink_to_fit(); // The branch size shall never changed. 
                    vertices[cur_unknown_length] = VertexState::Cache(peeked); 
                }
            },
            VertexState::Cache(_) => {
                // Reach the peeked branches. Push all these branch into processing queue.
                let nexts = vertices[offset].take();

                if let Some(nexts) = nexts {
                    // attempt to add each vertex to processing queue.
                    for v in nexts.iter() {
                        queue.push([i, *v, unknown_len]);

                        if *v >= text.len() {
                            // There is a vertex that already reach last char of text.
                            not_ended = false;
                            break
                        }
                    }
                    vertices[offset] = VertexState::Some(nexts); // Change state of vertex to Some
                }
            },
            VertexState::Some(_) => {
                // We need to update the link back if vertex count is equals and unknown word count is lower. 
                // So that it pick the path with least unknown word.
                // Since the result is construct based on queue and best result is a last vertex in queue,
                // it might point to a path that has more longer unknown token.
            }
        }

        i += 1;
    }

    let last_queue = queue.len() - 1;
    // Prune remain queue to see if there's any last vertex candidate with lesser unknown word.
    // Check only up to vertex before last element as last element is currently comparator vertex
    while i < queue.len() - 1 {
        let [_, offset, unknown_bytes] = queue[i];
        match vertices[offset] {
            VertexState::None | VertexState::Cache(_) => {},
            VertexState::Some(ref vs) => {
                if vs.iter().any(|v| {*v >= text.len()}) && unknown_bytes < queue[last_queue][2] {
                    // There's at least one edge point to last node with lesser unknown_bytes.
                    // Redirect last vertex reverse link to current vertex instead as it is new best vertex.
                    queue[last_queue][0] = i;
                }
            }
        }
        i += 1;
    }

    let [mut i, mut last_offset, _] = queue[last_queue]; // last element of queue
    while i > 0 {
        let [index, offset, _] = queue[i];
        // since offset is an offset of vertex and each vertex position designate a first char of word..
        result.push_front(&text[offset..last_offset]);
        last_offset = offset; // move offset to beginning of character

        i = index; // move index to another node in queue
    }

    // first word
    result.push_front(&text[0..last_offset]);
    
    result.into()
}

/// Dictionary based Thai text tokenizer
pub struct Tokenizer {
    dict: crate::dict::SizedDict,
}

impl Tokenizer {
    /// Construct a Thai tokenizer using given path as a dictionary.
    /// 
    /// Thai text tokenization rely on dictionary or corpus depending on algorithm being use
    /// to identify a token. 
    /// 
    /// In this implementation, we chose Dictionary approach and use "maximum matching" algorithm.
    /// The quality of tokenization depends on quality of dictionary.
    /// 
    /// The dictionary should be a text file where each line in text file must be individual word.
    /// The content of text file should have following format:
    /// ```txt
    /// กรุงเทพ
    /// กรุงเทพมหานคร
    /// จังหวัด
    /// ```
    pub fn new<P: AsRef<std::path::Path>>(dict_path: P) -> std::io::Result<Tokenizer> {
        Ok(Tokenizer {
            dict: crate::dict::Dict::load_txt(dict_path)?.into()
        })
    }
}

/// Create a tokenizer from slice of `&str` using the slice as dictionary.
impl From<&[&str]> for Tokenizer {
    fn from(slice: &[&str]) -> Tokenizer {
        let mut dict = crate::dict::Dict::new();
        slice.iter().for_each(|word| {dict.add(word)});
        Tokenizer {
            dict: dict.into()
        }
    }
}

/// Create a tokenizer from slice of `&String` using the slice as dictionary.
impl From<&[&String]> for Tokenizer {
    fn from(slice: &[&String]) -> Tokenizer {
        let mut dict = crate::dict::Dict::new();
        slice.iter().for_each(|word| {dict.add(word)});
        Tokenizer {
            dict: dict.into()
        }
    }
}

/// Create a tokenizer from slice of `String` using the slice as dictionary.
impl From<&[String]> for Tokenizer {
    fn from(slice: &[String]) -> Tokenizer {
        let mut dict = crate::dict::Dict::new();
        slice.iter().for_each(|word| {dict.add(word)});
        Tokenizer {
            dict: dict.into()
        }
    }
}

impl crate::tokenizer::Tokenizer for Tokenizer {
    fn tokenize<'b>(&self, value: &'b str) -> Vec<&'b str> {
        #[cfg(not(feature="single-thread"))]
        use rayon::iter::ParallelIterator;
        
        #[cfg(not(feature="single-thread"))]
        fn make_iter(raw: &str) -> rayon::str::SplitWhitespace {
            use rayon::prelude::*;

            raw.par_split_whitespace()
        }
        #[cfg(feature="single-thread")]
        fn make_iter(raw: &str) -> std::str::SplitWhitespace {
            raw.split_whitespace()
        }

        make_iter(value).map(|boundary| {
            // let mut leaf_nodes = Vec::new();
            // let root = TreeNode::root();
            // make_result_tree(&self.dict.root, boundary, root, &mut leaf_nodes);
            // let mut min_count = boundary.len(); // worst case length
            // let mut min_unknown = boundary.len(); // Impossible case where each char is treat as unknown token
            // let mut idx = 0;
            // // Maximum matching approach, find a path with least node
            // leaf_nodes.iter().enumerate().for_each(|(i, n)| {
            //     let level = n.node.level();
            //     let unknown = n.unknown_bytes_count;

            //     if unknown < min_unknown {
            //         min_unknown = unknown;

            //         if level <= min_count {
            //             // New best case, lower unknown and max matched
            //             min_count = level;
            //             idx = i;
            //         }
            //     } else {
            //         if level < min_count {
            //             // Subjective case, more unknown tokens but better matched
            //             // Need more sophisticate approach to solve this
            //         }
            //     }
            // });
            // let expected_node = leaf_nodes.remove(idx);
            // let result = expected_node.node.into_vec();
            // result
            maximal_matching(&self.dict.root, boundary)
        }).flatten().collect()
    }
}

#[cfg(test)]
mod tests;