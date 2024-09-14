mod tree;

use tree::bst::BST;

fn main() {
    let mut bst = BST::new();
    bst.insert_node(3_i32);
    bst.insert_node(3_i32);
    bst.insert_node(2);
    bst.insert_node(12);
    bst.insert_node(12);
    bst.insert_node(5);
    bst.insert_node(4);
    bst.insert_node(12);
    bst.insert_node(1);
    bst.insert_node(12);
    bst.insert_node(12);

    bst.inorder(|val| println!("{}", val));
}
