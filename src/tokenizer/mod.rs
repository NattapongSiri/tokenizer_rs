use std::sync::{Arc, RwLock, Weak};

/// Currently supported tree operation.
/// 
/// Since Rust tree is not allow to have cyclic relation ship thus we need to wrap
/// a node with `Weak` on one end and either `Rc` or `Arc` in another end.
/// Since both of that types required interior mutability to mutate data.
/// There's a dilemma on whether the trait signature shall be `&self` or `&mut self`.
/// 
/// In this case, we choose to avoid making a decision by consume `self` instead.
/// This is to make it very obvious that the implementor shall clone the parent node.
/// Otherwise, it will consume the parent node itself.
pub trait TreeOp<T> {
    /// Add a child node to tree. It will increment level but it will not increment unknown_count.
    fn add_child(self, value: T) -> Self;
}

/// A Tree node that hold possibles tokenization result.
/// 
/// The relation between parent and child node in Rust requires either `Rc` or `Arc`.
/// `Rc` is not thread safe. We don't know whether user want to pass this tree to other thread.
/// Thus, we need to wrap it in `Arc`.
/// 
/// Rust prohibit cyclic relationship. So either parent or child need to hold a `Weak` container type.
/// In current design, we choose to make a parent hold a `Weak` reference to childs. This is to make
/// it possible to partially drop some unused childs from parent.
/// 
/// The root node will have no value, nor parent. Thus value and parent must be wrap inside Option.
/// 
/// Since both `Arc` and `Rc` only allow share immutable owned value but we need to add child node.
/// We need to wrap it inside interior mutability kind of type. It's either `RefCell`, `Mutex`, or `RwLock`.
/// We don't know if user will need to pass this tree to any other thread so we need a thread safe type.
/// There can be multiple reader at the same times so it make more sense to put the value in `RwLock`.
/// 
/// The node shall also know their own level so user don't have to traverse entire tree to find out the
/// min and max depth of the tree. They only need to check on every leaves nodes.
#[derive(Debug)]
pub(crate) struct TreeNode<T> {
    /// Level of node in current tree. Root node is at level 0. Childs of root is at level 1.
    level: usize,
    /// Current value of current node. Each node shall represent exactly one token.
    /// Root node will not have value.
    value: Option<T>,

    /// Reference to parent node. If child node is not drop, the parent will always live.
    parent: Option<Arc<RwLock<TreeNode<T>>>>,
    /// Reference to childs of current node. It is possible that the child is already dropped.
    childs: Vec<Weak<RwLock<TreeNode<T>>>>
}

impl<T> TreeNode<T> {
    /// Since every tree operation require wrapping itself in `Arc<RwLock<>>`, it would
    /// make user have easier usage by simply return `Arc<RwLock<TreeNode<T>>>`.
    fn root() -> Arc<RwLock<TreeNode<T>>> {
        Arc::new(RwLock::new(TreeNode {
            level: 0,
            value: None,

            parent: None,
            childs: Vec::new()
        }))
    }
}

/// Directly implement `TreeOp<T>` for `Arc<RwLock<TreeNode<T>>>` so caller doesn't need
/// to toggle read and write lock at their end. The method will transparently do it and
/// release all the locks it required inside the method.
impl<T> TreeOp<T> for Arc<RwLock<TreeNode<T>>> {
    fn add_child(self, value: T) -> Arc<RwLock<TreeNode<T>>> {
        let level = self.read().unwrap().level;
        let child = Arc::new(RwLock::new(TreeNode {
            level: level + 1,
            value: Some(value),
            parent: Some(Arc::clone(&self)),
            childs: Vec::new()
        }));
        self.write().unwrap().childs.push(Arc::downgrade(&child));

        child
    }
}

/// Convert branch of tree from given node up to root node into a Vec<T>.
/// 
/// If the given node is a root node or the node has no value, it'll panic.
/// 
/// This is shallow type conversion thus `T` must implement `Copy`.
/// It is automatically implement for most of built-in Rust type, including borrowed value.
impl<T> std::convert::From<&TreeNode<T>> for Vec<T> where T: Copy {

    fn from(node: &TreeNode<T>) -> Vec<T> {
        let mut v = Vec::with_capacity(node.level);
        
        fn traverse_tree<T>(node: &Arc<RwLock<TreeNode<T>>>, vec: &mut Vec<T>) where T: Copy {
            let actual_node = node.read().unwrap();
            
            if let Some(ref parent) = actual_node.parent {
                traverse_tree(parent, vec);
                // Add value here as it is not a root node. 
                vec.push(*actual_node.value.as_ref().unwrap());
            }
        }

        if let Some(ref parent) = node.parent {
            traverse_tree(parent, &mut v);
        }
        
        if node.value.is_none() {
            panic!("The given node has no value. Either it is a root node or it is improper constructed node.");
        }

        v.push(*node.value.as_ref().unwrap());

        v.into()
    }
}

pub trait Tokenizer {
    /// Tokenize given `text` and return a `Vec<&str>` where each `&str` inside
    /// a `Vec` is a slice from given text.
    fn tokenize<'a>(&self, text: &'a str) -> Vec<&'a str>;
}

pub mod en;
pub mod th;

#[cfg(test)]
mod tests;