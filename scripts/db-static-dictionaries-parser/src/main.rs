mod datastruct;

use datastruct::BinaryTree;

// const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../dicts/db_static_dictionaries_001b1");
const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../dicts/db_static_dictionaries_034a0_00");

fn main() {
    let binary_tree = BinaryTree::new(DB_STATIC_DICTIONARIES);
    binary_tree.parse_n_print();
}
