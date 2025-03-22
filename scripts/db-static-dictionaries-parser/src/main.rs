mod datastruct;

use datastruct::BinaryTree;

// const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../dicts/db_static_dictionaries_001b");
const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../dicts/localization");

fn main() {
    let binary_tree = BinaryTree::new(DB_STATIC_DICTIONARIES);
    binary_tree.parse_n_print();
}

// fn main() {

//     let path =
//         std::path::Path::new("C:\\Projects\\survarium-001b\\survarium_full_v0100b\\resources");

//     _ = visit_dirs(path, &|path| cb(path));
// }

// fn cb(path: &std::fs::DirEntry) {
//     let result = std::panic::catch_unwind(|| {
//         let buffer = std::fs::read(path.path()).unwrap();
//         let binary_tree = BinaryTree::new(&buffer);
//         binary_tree.parse_n_print();
//     });
//     if result.is_err() {
//         std::eprintln!(
//             "Failed reading: {path}",
//             path = path.path().to_string_lossy(),
//         )
//     }
// }

// fn visit_dirs(dir: &std::path::Path, cb: &dyn Fn(&std::fs::DirEntry)) -> std::io::Result<()> {
//     if dir.is_dir() {
//         for entry in std::fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 visit_dirs(&path, cb)?;
//             } else {
//                 cb(&entry);
//             }
//         }
//     }
//     Ok(())
// }
