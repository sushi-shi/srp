use std::collections::HashMap;

#[derive(serde::Serialize)]
pub struct Skills(HashMap<String, Skill>);

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

#[derive(serde::Serialize, Debug)]
struct Booster {
    id: i32,
    value: f32,
}

impl Skills {
    pub fn build() -> Self {
        let mut perk_id = 1;

        let mut skills = HashMap::new();

        // 1перк: стрелковая подготовка
        // точность прицеливания: -2.50
        // скорость прицеливания:  5.00
        //
        // 2перк: физическая подготовка
        // воостановление энергии: 5.00
        // скорость передвижения: 0.50
        // переносимый вес: 1:00
        //
        // 3перк: инженерная подготовка
        // урон от гранат и ловушек: 2.50
        // радиус поражения: 1.00
        //
        // 4перк: выживание и первая помощь
        // заживление ран: 5.00
        // болевые ощущения: 2.00
        //
        // 5перк: знание мира
        // поиск артефактов: -5.00
        // урон от аномалий: -2.50

        for (skill_id, booster_ids) in [
            (1, vec![(1, -2.5), (2, 5.0)]),
            (2, vec![(4, 5.0), (5, 0.5), (6, 1.)]),
            (3, vec![(10, 2.5), (11, 1.0)]),
            (4, vec![(3, 5.0), (7, 2.0)]),
            (5, vec![(8, -5.0), (9, -2.5)]),
        ] {
            let mut levels = HashMap::new();

            for skill_level_id in 1..21 {
                let perks;
                let boosters = booster_ids
                    .iter()
                    .copied()
                    .map(|(id, boost)| Booster {
                        id,
                        value: (skill_level_id as f32) * boost,
                    })
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
