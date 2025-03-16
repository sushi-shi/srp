use crc32fast::Hasher;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
struct vostok_configs_binary_config_value {
    data: u32,
    id: u8,
    id_crc: u32,
    r#type: u16,
    count: u16,
}

const DB_STATIC_DICTIONARIES: &[u8] = include_bytes!("../db_static_dictionaries");

fn main() {
    let size =
        DB_STATIC_DICTIONARIES.len() / std::mem::size_of::<vostok_configs_binary_config_value>();

    println!("{size}");

    let slice = unsafe {
        std::slice::from_raw_parts(
            DB_STATIC_DICTIONARIES.as_ptr() as *const vostok_configs_binary_config_value,
            size,
        )
    };

    println!("{slice:?}");

    let mut hasher = Hasher::new();
    hasher.update(b"dict_id");
    let checksum = hasher.finalize();

    println!("{checksum}");

    let checksum = checksum.to_le_bytes();

    println!("{checksum:X?}");
}
