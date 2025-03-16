use crc32fast::Hasher;
use std::collections::BTreeSet;
use std::collections::HashSet;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct vostok_configs_binary_config_value {
    data: *mut std::ffi::c_void,
    id: u8,
    id_crc: u32,
    r#type: u16,
    count: u16,
    _padding: u32,
}
const _: () = assert!(std::mem::size_of::<vostok_configs_binary_config_value>() == 0x18);
const _: () = assert!(std::mem::align_of::<vostok_configs_binary_config_value>() == 0x8);

// 8
// 0             20      24
// [u64; u64; u32],<...>,[u64; u64; u32],

impl vostok_configs_binary_config_value {
    fn value_int(&self) -> u32 {
        assert_eq!(self.r#type, 1);

        let data = self.data as u64;
        u32::from_le_bytes(data.to_le_bytes()[4..8].try_into().unwrap())
    }
}

const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../db_static_dictionaries");

fn get_hash(name: &str) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(name.as_bytes());
    hasher.finalize()
}

fn search_all() {
    let checksum = get_hash("dict_id");

    visit_dirs(
        std::path::Path::new("C:\\Projects\\survarium-001b\\survarium_full_v0100b"),
        &|entry: &std::fs::DirEntry| {
            let path = entry.path();
            if path.is_file() {
                let contents = std::fs::read(&path).unwrap();
                if contents.is_empty() {
                    return;
                }
                let mut buffer = vec![];
                for i in 0..(contents.len() / 4) - 1 {
                    let value = u32::from_le_bytes(contents[4 * i..4 * i + 4].try_into().unwrap());
                    buffer.push(value)
                }

                if buffer.contains(&checksum) {
                    println!("Found path: {path}", path = path.to_string_lossy());
                }
            }
        },
    )
    .unwrap();
}
fn visit_dirs(dir: &std::path::Path, cb: &dyn Fn(&std::fs::DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    // search_all();

    let checksum = get_hash("dict_id");
    println!("{checksum}");
    {
        let checksum = checksum.to_le_bytes();
        println!("{checksum:X?}");
    }

    let sentinel = u32::from_le_bytes(DB_STATIC_DICTIONARIES[0..4].try_into().unwrap());
    assert_eq!(sentinel, 0x18);

    let db_static_dictionaries = &DB_STATIC_DICTIONARIES[4..];

    let mut buffer: Vec<u64> = Vec::with_capacity(db_static_dictionaries.len() / 8);
    for i in 0..(db_static_dictionaries.len() / 8) - 1 {
        let value =
            u64::from_le_bytes(db_static_dictionaries[8 * i..8 * i + 8].try_into().unwrap());
        buffer.push(value)
    }

    let size =
        db_static_dictionaries.len() / std::mem::size_of::<vostok_configs_binary_config_value>();

    println!("{size}");

    let slice = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr() as *const vostok_configs_binary_config_value,
            size,
        )
    };

    // for i in 1..40 {
    //     println!("{:?}", slice[i]);
    // }

    let mut actual_ids = BTreeSet::new();
    let mut found_ids = HashSet::new();
    let values = slice
        .iter()
        .enumerate()
        .filter(|(_, value)| value.id_crc == checksum);
    let mut len = 0;
    for (i, value) in values {
        let offset = i * std::mem::size_of::<vostok_configs_binary_config_value>();
        let offset_end = offset + std::mem::size_of::<vostok_configs_binary_config_value>();
        len += 1;
        let prev_value = slice[i - 1];
        if !actual_ids.insert(prev_value._padding) {
            panic!("{prev_value:?}");
        }
        println!("[=] {prev_value:?}",);
        println!(
            "[+] 0x{offset:X}..0x{offset_end:X}: {value:?} -- {}",
            value.value_int()
        );
        found_ids.insert(offset + 8 + 4);

        let int = value.value_int() as usize;
        let huh = u32::from_le_bytes(db_static_dictionaries[int..int + 4].try_into().unwrap());
        println!("{huh}");
    }
    println!("Total dict_ids: {len}");
    println!("Actual IDS: {actual_ids:?}");

    let mut actual_len = 0;
    for i in 0..db_static_dictionaries.len() - 4 {
        let id_crc = u32::from_le_bytes(db_static_dictionaries[i..i + 4].try_into().unwrap());
        if id_crc == checksum {
            actual_len += 1;
            if !found_ids.contains(&i) {
                println!("0x{i:X}");
            } else {
                println!("0x{i:X} -- FOUND");
            }
        }
    }
    println!("Actual total dict_ids: {actual_len}");
}
