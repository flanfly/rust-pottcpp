use std::boxed::Box;
use std::ops::Deref;

enum Tree {
	Leaf(char),
	Node(Box<Tree>,Box<Tree>)
}

fn depth_first_search(root: &Tree) {
	match root {
		&Tree::Leaf(s) => println!("{}",s),
		&Tree::Node(ref left,ref right) => {
			depth_first_search(left.deref());
			depth_first_search(right.deref())
		}
	}
}

fn main() {
    let tree =
      Box::new(Tree::Node(
        Box::new(Tree::Node(
          Box::new(Tree::Leaf('H')),
          Box::new(Tree::Node(
            Box::new(Tree::Leaf('e')),
            Box::new(Tree::Leaf('l')))))),
        Box::new(Tree::Node(
          Box::new(Tree::Node(
            Box::new(Tree::Leaf('l')),
            Box::new(Tree::Leaf('o')))),
          Box::new(Tree::Leaf('!'))))));

    // Prints "Hello!"
    depth_first_search(&tree);
}
