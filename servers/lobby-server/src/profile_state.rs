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

#[allow(non_camel_case_types)]
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

impl player_profile {
    pub fn new() -> Self {
        let mut slots = [inventory_item_instance::default(); 19];
        slots[0] = inventory_item_instance {
            condition_or_stack: 0,
            amount_in_inventory: 1,
            id: 1,
            dict_id: 55,
        };
        slots[1] = inventory_item_instance {
            condition_or_stack: 0,
            amount_in_inventory: 1,
            id: 2,
            dict_id: 35,
        };
        Self {
            account_id: 1,
            profile_id: 2,
            profile_name: *b"hello\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            boosters: [skill_booster::default(); 11],
            slots,
            team: game_team_id::team_neutral,
            is_local: false,
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
