mod deserialize;
mod serialize;
mod skills_tree;

use skills_tree::Skills;

fn main() {
    let skills_tree = Skills::build();
    let skills_tree = serde_json::to_value(skills_tree).unwrap();

    let binary_tree = serialize::parse_json_into_binary_tree(&skills_tree);
    std::fs::write("./resources/skills_tree.bin", binary_tree.as_bytes()).unwrap();

    binary_tree.parse_n_print();
}
