use tree2::Tree;

fn main() {
    let mut tree = Tree::new();
    for i in (0..100).rev() {
        tree.insert(i);
    }

    for x in tree.iter() {
        println!("{x}");
    }
}
