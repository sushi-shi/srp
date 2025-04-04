use crc32fast::Hasher;
use encoding_rs::WINDOWS_1251;
use num_traits::FromPrimitive;
use std::ffi::CStr;

/// Constraints:
/// 1. The first `BinaryValue` should always be:
/// ```ignore
/// { data: 0x18, id: 0, id_crc: "", type_: t_table_named, count: 5 }
/// ```
/// 2. The data structure should be consistent with all offsets pointing inside it
pub struct BinaryConfig(Vec<u8>);

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BinaryValue {
    /// Either a value or offset from root to a table
    pub data: u64,
    pub id: u64,
    pub id_crc: IdCrc,
    pub type_: BinaryType,
    pub count: u16,
}
const _: () = assert!(std::mem::size_of::<BinaryValue>() == 0x18);
const _: () = assert!(std::mem::align_of::<BinaryValue>() == 0x8);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, num_derive::FromPrimitive)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum BinaryType {
    t_boolean,
    t_integer,
    t_float,
    t_table_named,
    t_table_indexed,
    t_string,
    t_float2,
    t_float3,
    t_float4,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[repr(transparent)]
pub struct IdCrc(u32);

impl IdCrc {
    pub fn get(self) -> u32 {
        self.0
    }

    pub fn get_hash(name: &str) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(name.as_bytes());
        Self(hasher.finalize())
    }
}

impl BinaryConfig {
    pub fn new(binary: &[u8]) -> Self {
        assert!(binary.len() >= 0x18);
        Self(binary.to_vec())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn parse_n_print(&self) {
        let buffer = &self.0;
        let value = BinaryValue::parse(buffer);
        value.print_rec(self, 0, None);
    }

    #[allow(dead_code)]
    pub fn parse_n_print_str(&self) {
        let buffer = &self.0;
        let value = BinaryValue::parse(buffer);
        value.print_str_rec(self);
    }
}

impl BinaryValue {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn parse(buffer: &[u8]) -> Self {
        assert!(buffer.len() >= Self::SIZE);

        let buffer: [u8; Self::SIZE] = buffer[0..Self::SIZE].try_into().unwrap();
        let (data, id, id_crc, type_, count) = arrayref::array_refs![&buffer, 8, 8, 4, 2, 2];

        let data = u64::from_le_bytes(*data);
        let id = u64::from_le_bytes(*id);
        let id_crc = u32::from_le_bytes(*id_crc);
        let type_ = u16::from_le_bytes(*type_);
        let count = u16::from_le_bytes(*count);

        let id_crc = IdCrc(id_crc);
        let type_ = BinaryType::from_u16(type_).unwrap();

        Self {
            data,
            id,
            id_crc,
            type_,
            count,
        }
    }

    fn print_rec(&self, tree: &BinaryConfig, depth: usize, index: Option<usize>) {
        let prefix = match index {
            Some(index) => {
                let tab = tab(depth);

                format!("{tab}{index}")
            }
            None => {
                let tab = tab(depth);

                let id = &tree.0[self.id as usize..];
                let id_crc = CStr::from_bytes_until_nul(id).unwrap().to_str().unwrap();

                format!("{tab}{id_crc}")
            }
        };

        match self.type_ {
            BinaryType::t_table_named | BinaryType::t_table_indexed => {
                let offset = self.data as usize;
                let len = self.count as usize;

                println!("{prefix}[{len}]");

                let buffer = &tree.0;
                for i in 0..len {
                    let this = Self::parse(&buffer[offset + i * Self::SIZE..]);

                    let index = match self.type_ {
                        BinaryType::t_table_named => None,
                        BinaryType::t_table_indexed => Some(i),
                        _ => unreachable!(),
                    };

                    Self::print_rec(&this, tree, depth + 1, index);
                }
            }

            BinaryType::t_boolean => {
                let value = self.data != 0;
                println!("{prefix}: {value}");
            }
            BinaryType::t_integer => {
                let value = self.data as i32;
                println!("{prefix}: {value}");
            }
            BinaryType::t_float => {
                let value = f32::from_bits(self.data as u32);
                println!("{prefix}: {value}");
            }
            BinaryType::t_string => {
                let offset = self.data as usize;
                let len = self.count as usize;
                let buffer = &tree.0[offset..offset + len - 1]; // '\0'

                let (value, _, had_errors) = WINDOWS_1251.decode(buffer);
                assert!(!had_errors);
                // let value = String::from_utf8_lossy(buffer);

                println!("{prefix}: \"{value}\"");
            }
            BinaryType::t_float2 => {
                let offset = self.data as usize;
                let buffer = tree.0[offset..offset + 4 * 2].try_into().unwrap();

                let (float_x, float_y) = arrayref::array_refs![&buffer, 4, 4];
                let float_x = f32::from_le_bytes(*float_x);
                let float_y = f32::from_le_bytes(*float_y);
                println!("{prefix}: {float_x}|{float_y}");
            }
            BinaryType::t_float3 => {
                let offset = self.data as usize;
                let buffer = tree.0[offset..offset + 4 * 3].try_into().unwrap();

                let (float_x, float_y, float_z) = arrayref::array_refs![&buffer, 4, 4, 4];
                let float_x = f32::from_le_bytes(*float_x);
                let float_y = f32::from_le_bytes(*float_y);
                let float_z = f32::from_le_bytes(*float_z);
                println!("{prefix}: {float_x}|{float_y}|{float_z}");
            }
            BinaryType::t_float4 => {
                let offset = self.data as usize;
                let buffer = tree.0[offset..offset + 4 * 4].try_into().unwrap();

                let (float_x, float_y, float_z, float_w) =
                    arrayref::array_refs![&buffer, 4, 4, 4, 4];
                let float_x = f32::from_le_bytes(*float_x);
                let float_y = f32::from_le_bytes(*float_y);
                let float_z = f32::from_le_bytes(*float_z);
                let float_w = f32::from_le_bytes(*float_w);
                println!("{prefix}: {float_x}|{float_y}|{float_z}|{float_w}");
            }
        }
    }

    fn print_str_rec(&self, tree: &BinaryConfig) {
        match self.type_ {
            BinaryType::t_table_named | BinaryType::t_table_indexed => {
                let offset = self.data as usize;
                let len = self.count as usize;

                let buffer = &tree.0;
                for i in 0..len {
                    let this = Self::parse(&buffer[offset + i * Self::SIZE..]);
                    Self::print_str_rec(&this, tree);
                }
            }

            BinaryType::t_string => {
                let offset = self.data as usize;
                let len = self.count as usize;
                let buffer = &tree.0[offset..offset + len - 1]; // '\0'

                let (value, _, had_errors) = WINDOWS_1251.decode(buffer);
                assert!(!had_errors);

                println!("{value}");
            }
            BinaryType::t_boolean
            | BinaryType::t_integer
            | BinaryType::t_float
            | BinaryType::t_float2
            | BinaryType::t_float3
            | BinaryType::t_float4 => {}
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, Self::SIZE) }
    }
}

fn tab(depth: usize) -> String {
    (0..depth).map(|_| "  ").collect()
}
