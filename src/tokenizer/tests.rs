use super::*;


#[cfg(not(feature="single-thread"))]
#[test]
#[should_panic(expected="The given node has no value. Either it is a root node or it is improper constructed node.")]
fn test_tree() {
    let root = TreeNode::root();
    let a = Arc::clone(&root).add_child("a");
    let one = Arc::clone(&a).add_child("1");
    let two = Arc::clone(&a).add_child("2");
    let b = Arc::clone(&root).add_child("b");
    assert_eq!(Vec::from(&*two.read().unwrap()), vec!["a", "2"]);
    assert_eq!(Vec::from(&*one.read().unwrap()), vec!["a", "1"]);
    assert_eq!(Vec::from(&*b.read().unwrap()), vec!["b"]);

    // This should cause panic as root node has no value
    Vec::<&str>::from(&*TreeNode::root().read().unwrap());
}
#[cfg(feature="single-thread")]
#[test]
#[should_panic(expected="The given node has no value. Either it is a root node or it is improper constructed node.")]
fn test_tree() {
    let root = TreeNode::root();
    let a = Rc::clone(&root).add_child("a");
    let one = Rc::clone(&a).add_child("1");
    let two = Rc::clone(&a).add_child("2");
    let b = Rc::clone(&root).add_child("b");
    assert_eq!(Vec::from(&*two.borrow()), vec!["a", "2"]);
    assert_eq!(Vec::from(&*one.borrow()), vec!["a", "1"]);
    assert_eq!(Vec::from(&*b.borrow()), vec!["b"]);

    // This should cause panic as root node has no value
    Vec::<&str>::from(&*TreeNode::root().borrow());
}