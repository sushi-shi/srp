#![expect(dead_code)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct player_profile {
    pub account_id: u32,
    pub profile_id: u32,
    pub profile_name: [u8; 32],
    pub boosters: [skill_booster; 11],
    pub slots: [inventory_item_instance; 19],
    pub team: game_team_id,
    pub is_local: bool,
}
const _: () = assert!(std::mem::size_of::<player_profile>() == 0x1B8);

#[expect(non_camel_case_types)]
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum game_team_id {
    team_1 = 0x0,
    team_2 = 0x1,
    team_neutral = 0x2,
    team_undefined = 0x3,
    team_invalid = 0xFF,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct skill_booster {
    pub id: u8,
    pub value: f32,
}
const _: () = assert!(std::mem::size_of::<skill_booster>() == 8);

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct inventory_item_instance {
    pub condition_or_stack: u32,
    pub amount_in_inventory: u32,
    pub id: u32,
    pub dict_id: u16,
}
const _: () = assert!(std::mem::size_of::<inventory_item_instance>() == 0x10);

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[expect(non_camel_case_types)]
#[repr(u8)]
enum profile_slot_enum {
  helmet_slot        = 0x0,
  mask_slot          = 0x1,
  torso_slot         = 0x2,
  back_slot          = 0x3,
  pants_slot         = 0x4,
  gloves_slot        = 0x5,
  boots_slot         = 0x6,
  weapon1_slot       = 0x7,
  ammo1_weapon1_slot = 0x8,
  ammo2_weapon1_slot = 0x9,
  weapon2_slot       = 0xA,
  ammo1_weapon2_slot = 0xB,
  ammo2_weapon2_slot = 0xC,
  quick_slot1        = 0xD,
  quick_slot2        = 0xE,
  quick_slot3        = 0xF,
  quick_slot4        = 0x10,
  quick_slot5        = 0x11,
  quick_slot6        = 0x12,
  max_slots_count    = 0x13,
}

impl<T, const N: usize> std::ops::Index<profile_slot_enum> for [T; N] {
    type Output = T;

    fn index(&self, index: profile_slot_enum) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T, const N: usize> std::ops::IndexMut<profile_slot_enum> for [T; N] {
    fn index_mut(&mut self, index: profile_slot_enum) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl player_profile {
    pub fn new(account_id: u32, profile_id: u32, profile_name: &str) -> Self {
        let i = |id, dict_id, condition_or_stack| inventory_item_instance {
            condition_or_stack,
            amount_in_inventory: 1,
            id,
            dict_id,
        };

        let mut slots = [inventory_item_instance::default(); 19];

        #[rustfmt::skip]
        {
            use profile_slot_enum::*;
            slots[boots_slot]   = i(1, 24, 10);
            slots[gloves_slot]  = i(2, 40, 20);
            slots[pants_slot]   = i(3, 46, 30);
            slots[helmet_slot]  = i(4, 27, 40);
            slots[mask_slot]    = i(5, 43, 50);
            slots[torso_slot]   = i(6, 48, 60);
            slots[back_slot]    = i(7, 9,  70);
            slots[weapon1_slot] = i(12, 55, 120);
            slots[weapon2_slot] = i(12, 55, 130);

            // slots[ammo1_weapon1_slot] = i(...);
            // slots[ammo2_weapon1_slot] = i(...);
            // slots[ammo1_weapon2_slot] = i(...);
            // slots[ammo2_weapon2_slot] = i(...);
        };

        if 3 > profile_name.len() || profile_name.len() > 30 {
            panic!("bad name")
        }

        let profile_name = {
            let mut bytes = [b'\0'; 32];
            bytes[0..profile_name.len()].copy_from_slice(profile_name.as_bytes());
            bytes
        };

        Self {
            account_id,
            profile_id,
            profile_name,
            boosters: [skill_booster::default(); 11],
            slots,
            team: game_team_id::team_neutral,
            is_local: true,
        }
    }

    pub fn deserialize(&self) -> &[u8] {
        let ptr = self as *const _ as *const u8;
        let len = std::mem::size_of::<Self>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}
impl inventory_item_instance {
    pub fn serialize(&self) -> &[u8] {
        let ptr = self as *const _ as *const u8;
        let len = std::mem::size_of::<Self>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[expect(non_camel_case_types)]
#[repr(u8)]
pub enum skill_booster_enum {
    empty                         = 0,
    st_dispersion_correction      = 1,
    st_aiming_speed_correction    = 2,
    st_health_regen_correction    = 3,
    st_stamina_regen_correction   = 4,
    st_movement_speed_correction  = 5,
    st_additional_max_weight_name = 6,
    st_pain_healt_correction      = 7,
    st_artcontainer_time_corr     = 8,
    st_anomaly_damage_corr        = 9,
    st_engineer_use_time_corr     = 10,
    st_engineer_succ_chance_corr  = 11,
}
