mod datastruct;
mod temt;

use std::collections::HashMap;

use datastruct::BinaryTree;

#[expect(dead_code)]
const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../dicts/db_static_dictionaries_001b");

fn main() {
    let skills_tree = Skills::build();
    let skills_tree = serde_json::to_value(skills_tree).unwrap();

    let binary_tree = temt::parse_json_into_binary_tree(&skills_tree);
    std::fs::write("./skills_tree.bin", &binary_tree).unwrap();

    BinaryTree::new(&binary_tree).parse_n_print();
}

#[derive(serde::Serialize)]
struct Skills(HashMap<String, Skill>);

#[derive(serde::Serialize)]
struct Skill {
    id: i32,
    levels: Levels,
}

#[derive(serde::Serialize)]
struct Levels(HashMap<String, SkillLevel>);

#[derive(serde::Serialize)]
struct SkillLevel {
    boosters: Vec<Booster>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    perks: Vec<Perk>,
}

#[derive(serde::Serialize)]
struct Perk {
    id: i32,
}

#[derive(serde::Serialize)]
struct Booster {
    id: i32,
    value: f32,
}

impl Skills {
    fn build() -> Self {
        let mut perk_id = 1;

        let mut skills = HashMap::new();

        for (skill_id, booster_ids) in [
            (1, vec![1, 2]),
            (2, vec![4, 5, 6]),
            (3, vec![10, 11]),
            (4, vec![3, 7]),
            (5, vec![8, 9]),
        ] {
            let mut levels = HashMap::new();

            for skill_level_id in 1..21 {
                let perks;
                let boosters = booster_ids
                    .iter()
                    .copied()
                    .map(|id| Booster { id, value: 100. })
                    .collect();

                match () {
                    () if skill_level_id == 20 => {
                        perks = vec![Perk { id: perk_id }];

                        perk_id += 1;
                    }
                    () if skill_level_id % 5 == 0 => {
                        perks = vec![Perk { id: perk_id }, Perk { id: perk_id + 1 }];

                        perk_id += 2;
                    }
                    () => {
                        perks = vec![];
                    }
                };

                levels.insert(
                    format!("skill_level_{skill_level_id}"),
                    SkillLevel { boosters, perks },
                );
            }

            skills.insert(
                format!("skill_{skill_id}"),
                Skill {
                    id: skill_id,
                    levels: Levels(levels),
                },
            );
        }

        Self(skills)
    }
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
