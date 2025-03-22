mod datastruct;

use datastruct::BinaryTree;

// const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../dicts/player_death.particle");
const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../dicts/db_static_dictionaries_001b");

fn main() {
    let binary_tree = BinaryTree::new(DB_STATIC_DICTIONARIES);
    binary_tree.parse_n_print();
}
