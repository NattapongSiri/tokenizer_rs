/// Find a node that has longest common prefix matched with given value.
/// It return index of the node and the length of the matched.
/// It assume that the node is sorted in ascending order.
/// 
/// This doesn't mean that it is exactly match to the node.
/// For example, the matched node may have value `aab` and value may be `aac`.
/// In such case, it will return index to the node and the lenght will be 2.
/// 
/// It is important to note that return length is in bytes so that
/// caller can directly take slice from original value to get a 
/// longest prefix.
/// 
/// The function guarantee that the length will be valid string.
/// 
/// Example interpretation of return value:
/// - (0, 0) - There's no common prefix node value with given value, new node may be added at 0
/// - (3, 0) - There's no common prefix node value with given value, new node may be added at 3
/// 
/// If user want to add new node so it can match this value in the future, user must add
/// it at given suggestion. Otherwise, this function will break.
/// - (0, 3) - The first node has common prefix of length 3 with given value.
/// - (3, 1) - The third node has one character common prefix with given value.
/// 
/// The index will always `< nodes.len()` and the length will always `<=` node value
fn find_longest_prefix(nodes: &[Node], value: &str) -> (usize, usize) {
    let mut index = nodes.len();
    let mut longest = 0;
    let value_first_char = value.chars().next();

    for (i, node) in nodes.iter().enumerate() {
        let mut n = 0;

        for (nv, cv) in node.value.chars().zip(value.chars()) {
            if nv != cv {
                break;
            }
            n += nv.len_utf8();
        }

        if n > longest {
            // new common prefix with longer that previous one found.
            index = i;
            longest = n;
        } else if node.value.chars().next() > value_first_char && longest == 0 {
            // The node slice shall be sorted.
            // If there's no commom prefix with current node and it is not the first node and there
            // is no previously matched common prefix then we only need to check the first character 
            // of the node if the character order is after the given value, it shall not continue further lookup.
            // The reason it need only first character is because if the node's first character order
            // came before the first cahracter of value, it mean there's a chance that next node may
            // have common prefix with the value. If it has equals first character then it must have
            // at least one common prefix. If it order is after the value, it has no chance to found
            // any node with common prefix. Therefore, it shall return the current index as it is
            // the first node that will be immediate after the value
            index = i;
            break
        }
    };

    (index, longest)
}

/// Attempt to merge a child node if the given node has only 1 child.
/// It will work recursively until first node with > 1 or leaf node reach.
/// 
/// It is unlikely to happen if entire tree is construct via `add` or `load` method.
#[allow(unused)]
fn try_merge(node: &mut Node) {
    if node.terminal {
        // Cannot collapse terminal node
        return
    }

    // only node with exactly one child can be collapsed
    if let Some(ref mut childs) = node.childs {
        if childs.len() != 1 {
            return
        }
    } else {
        return
    };

    // There's exactly one child of this node.
    let mut childs = node.childs.take().unwrap();
    try_merge(&mut childs[0]); // traverse until either hit leaf node or found a node with multiple child
    node.value.push_str(&childs[0].value);
    node.terminal = childs[0].terminal; // node type shall be propagate back to parent when collapsed
    node.childs = childs[0].childs.take();
}

/// Add value to given nodes while maintaining the ascending order of nodes.
/// It's always succeed.
fn add_node(nodes: &mut Vec<Node>, value: String) {
    let (i, len) = find_longest_prefix(&*nodes, &value);

    if len == 0 {
        // new node at current level
        nodes.insert(i, Node {childs: Some(vec![]), terminal: true, value: value});
    } else {
        // Four possibilities here.
        // 1. Node is prefix of given value
        // 2. Node is a given value
        // 3. Given value is a prefix of node.
        // 4. There's common prefix on both node value and given value.
        let node_len = nodes[i].value.len();
        let value_len = value.len();

        if len == node_len {
            // 100 % match with node value
            if len == value_len {
                // 100% match on both node_value and given value
                nodes[i].terminal = true;
            } else {
                // Node is prefix of given value as it is impossible to have len > value

                // add remain of value as child of current node
                add_node(nodes[i].childs.as_mut().unwrap(), value[len..].to_owned());
            }
        } else {
            // Prefix of node value match as it is impossible to have length > node_len
            if len >= value_len {
                // Given value is prefix of node value
                let remain = nodes[i].value[len..].to_owned(); // take all remain of node value
                nodes[i].value = nodes[i].value[..len].to_owned(); // truncate current node value to given value
                
                let child_of_childs = nodes[i].childs.take(); // move all childs out of current node
                let child = Node { // create new child to represent current node value
                    childs: child_of_childs, // move all childs back to restore represent current node's childs
                    terminal: nodes[i].terminal, // it shall have similar node type to original of it type
                    value: remain
                }; 
                nodes[i].childs = Some(vec![child]); // add a represent of current node as child of given value
                nodes[i].terminal = true; // since node value is equal to given value, it's terminal node
            } else {
                // there's a common prefix on both node value and given value.
                let node_remain = nodes[i].value[len..].to_owned(); // remain of node value
                nodes[i].value = nodes[i].value[..len].to_owned(); // truncate node value to represent common prefix.
                let value_remain = value[len..].to_owned(); // remain of given value
                let child_of_childs = nodes[i].childs.take(); // move all childs out of current node
                let child = Node { // create new child to represent current node value
                    childs: child_of_childs, // move all childs back to restore represent current node's childs
                    terminal: nodes[i].terminal, // it shall have similar node type to original of it type
                    value: node_remain
                };
                let mut childs = vec![child]; // construct sibling to be re-attached to current node
                add_node(&mut childs, value_remain); // add remain value as sibling of remain of current node
                nodes[i].childs = Some(childs); // reconnect all childs back
                nodes[i].terminal = false // It is no longer terminal as it is just a prefix of two nodes
            }
        }
    }
}

/// A mutable dictionary dictionary.
/// It is used as root of many childs [Node](struct.Node.html).
#[derive(Debug, PartialEq)]
pub(crate) struct Dict {
    root: Vec<Node>
}

impl Dict {
    /// Create new empty dictionary
    pub fn new() -> Dict {
        Dict {
            root: Vec::new()
        }
    }

    /// Load dictionary from text file
    pub fn load_txt<P: AsRef<std::path::Path>>(txt_file: P) -> std::io::Result<Dict> {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(std::fs::File::open(txt_file)?);
        let mut dict = Dict::new();
        reader.lines().for_each(|line| {
            dict.add(line.as_ref().unwrap());
        });
        Ok(dict)
    }

    /// Add new token into dictionary.
    /// The value will be clone and owned by this object.
    pub fn add(&mut self, value: &str) {
        add_node(&mut self.root, value.to_owned());
    }
}

/// A fixed number of elements dictionary.
/// 
/// It let user use method [matcher](struct.SizedDict.html#method.matcher) to
/// match multiple possible occurences of word in dictionary to a given string.
/// 
/// The different from [Dict][struct.Dict.html] is that you cannot add
/// more word into dict.
/// 
/// It is possible to mutate each node value inside a dict. However,
/// it is highly discouraged. The reason is because the dict is represented by sorted 
/// prefix tree data structure. You must  take extra precaution for effect on each mutation. That is:
/// 1. The mutation must keep the order of nodes in that layer. Otherwise, it will cause
/// invalid node traversal.
/// 1. The mutation will have effect on both upward and downward direction of the tree value.
///
/// It is easier to just create a new dict.
#[derive(Debug, PartialEq)]
pub(crate) struct SizedDict {
    pub(crate) root: Box<[SizedNode]>
}

/// Convert mutable dict into immutable.
impl core::convert::From<Dict> for SizedDict {
    fn from(dict: Dict) -> SizedDict {
        SizedDict {
            root: dict.root.into_iter().map(|n| n.into()).collect::<Vec<SizedNode>>().into_boxed_slice()
        }
    }
}

/// A fully mutable node that let user modify any value.
#[derive(Debug, PartialEq)]
struct Node {
    childs: Option<Vec<Node>>,
    terminal: bool,
    value: String,
}

/// Convert Node into SizedNode
impl core::convert::From<Node> for SizedNode {
    fn from(node: Node) -> SizedNode {
        SizedNode {
            childs: node.childs.unwrap_or(vec![])
                                .into_iter().map(|c| c.into())
                                .collect::<Vec<SizedNode>>()
                                .into_boxed_slice(),
            terminal: node.terminal,
            value: node.value
        }
    }
}

/// A fix sized node.
/// 
/// The only different from [Node](struct.Node.html) is that it have
/// fixed childs. That mean it cannot add, edit, or remove a child node.
#[derive(Debug, PartialEq)]
pub(crate) struct SizedNode {
    childs: Box<[SizedNode]>,
    terminal: bool,
    value: String,
}

/// Return all the nodes that is prefixed of value along with the remaining unmatched part.
/// The return value is a form of `Vec<(&SizedNode, &str)>`
/// 
/// # Parameters
/// - `nodes` - A slice of nodes to attempt to check if it is prefix of value
/// - `value` - A `&str` to find a prefix.
#[inline(always)]
fn childs_matched<'a, 'b>(nodes: &'a [SizedNode], value: &'b str) -> Vec<(&'a SizedNode, &'b str)> {
    nodes.iter().filter_map(|n| 
        if value.starts_with(&*n.value) {
            Some((n, &value[n.value.len()..]))
        } else {
            None
        }).collect()
}

/// Get an index to last chars of each possible valid word prefix from given value.
/// The result will be sorted by length of offset. That is the last element in `results` will always
/// be longest.
/// 
/// In typical use, caller shall call by:
/// `terminals_prefix(dict.root, "SOME_TEXT", 0, &mut vec_results)`
/// 
/// # Parameters
/// - `nodes` - A slice of [SizedNode](struct.SizedNode.html) to try to match with value
/// - `value` - A &str to find a prefix word
/// - `offset` - Usize of byte value. Caller usually give 0 to find a prefix from start of the text. 
/// - `results` - A mutable reference to Vec to hold an offset of last character of valid prefixed terminal nodes.
/// The offset unit is bytes so caller can take a slice using this offset on the string.
/// 
/// # Return
/// This function return value in last function parameter. That is `results: &mut Vec<usize>`.
pub(crate) fn terminals_prefix(nodes: &[SizedNode], value: &str, offset: usize, results: &mut Vec<usize>) {
    // A queue of pair of nodes and value to be evaluate.
    let mut eval_queue = std::collections::VecDeque::new();
    eval_queue.push_back((nodes, &value[offset..], offset));

    // Pop out nodes and value to be evaluate
    while let Some((nodes, value, offset)) = eval_queue.pop_front() {
        for (child, remain) in childs_matched(nodes, value) {
            // On each match, check if it is terminal and recursively check the childs node
            
            // We need to store new_offset as each char may have different length.
            // It is more expensive to calculate the offset backward than to simply just store it.
            let new_offset = offset + child.value.len();
            
            if child.terminal {
                // It is terminal node, to results vec
                results.push(new_offset);
            }

            if child.childs.len() > 0 && remain.len() > 0 {
                // Put all the childs and their remain to evaluation queue
                eval_queue.push_back((&*child.childs, remain, new_offset));
            }
        }
    }
}

#[cfg(test)]
mod tests;