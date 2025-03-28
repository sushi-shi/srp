use crate::deserialize::{BinaryConfig, BinaryType, BinaryValue, IdCrc};

use std::collections::{HashMap, VecDeque};

pub fn parse_json_into_binary_tree(root: &serde_json::Value) -> BinaryConfig {
    use serde_json::Value;

    let len = count_elements(root);
    let mut buffer = vec![0_u8; len * std::mem::size_of::<BinaryValue>()];

    let mut idx = 0;
    let mut names: HashMap<&str, (u64, IdCrc)> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((root, None, 0, IdCrc::default()));

    while let Some((value, parent_idx, id, id_crc)) = queue.pop_front() {
        if let Some(parent_idx) = parent_idx {
            fix_parent(&mut buffer, parent_idx, idx);
        }

        let value = match value {
            Value::Null => unreachable!(),
            Value::Bool(data) => {
                let data: u8 = (*data).into();

                BinaryValue {
                    data: data as u64,
                    id,
                    id_crc,
                    type_: BinaryType::t_boolean,
                    count: 0x1,
                }
            }
            Value::Number(data) => {
                if let Some(data) = data.as_i64() {
                    let data: i32 = data.try_into().unwrap();
                    BinaryValue {
                        data: data as u64,
                        id,
                        id_crc,
                        type_: BinaryType::t_integer,
                        count: 0x4,
                    }
                } else if let Some(data) = data.as_f64() {
                    let data: u32 = (data as f32).to_bits();

                    BinaryValue {
                        data: data as u64,
                        id,
                        id_crc,
                        type_: BinaryType::t_float,
                        count: 0x4,
                    }
                } else {
                    unreachable!()
                }
            }

            // @NOTE: We are mixing values with indexes, which we might not want to do!
            // Survarium devs did it in the same way, so whatever.
            Value::String(data) => {
                let offset = buffer.len() as u64;
                buffer.extend(data.as_bytes());
                buffer.push(0);

                let count: u16 = (data.len() + 1).try_into().unwrap();
                let data = offset;

                BinaryValue {
                    data,
                    id,
                    id_crc,
                    type_: BinaryType::t_string,
                    count,
                }
            }
            Value::Array(values) => {
                let mut parent_idx = Some(idx);
                queue.extend(values.iter().enumerate().map(|(id, v)| {
                    let parent_idx = parent_idx.take();
                    (v, parent_idx, id as u64, IdCrc::default())
                }));

                let data = 0;
                let count = values.len().try_into().unwrap();

                BinaryValue {
                    data,
                    id,
                    id_crc,
                    type_: BinaryType::t_table_indexed,
                    count,
                }
            }
            Value::Object(values) => {
                let mut values = values
                    .into_iter()
                    .map(|(k, v)| {
                        let (id, id_crc) = *names.entry(k).or_insert_with_key(|name| {
                            let offset = buffer.len() as u64;
                            buffer.extend(name.as_bytes());
                            buffer.push(0);
                            let crc = IdCrc::get_hash(k);

                            (offset, crc)
                        });

                        (v, None, id, id_crc)
                    })
                    .collect::<Vec<_>>();
                values.sort_by_key(|v| v.3.get());

                if let Some(value) = values.get_mut(0) {
                    value.1 = Some(idx)
                }

                let data = 0;
                let count = values.len().try_into().unwrap();

                queue.extend(values.into_iter());

                BinaryValue {
                    data,
                    id,
                    id_crc,
                    type_: BinaryType::t_table_named,
                    count,
                }
            }
        };
        write_value(&mut buffer, idx, value);
        idx += 1;
    }

    BinaryConfig::new(&buffer)
}

pub fn count_elements(input: &serde_json::Value) -> usize {
    let inner_items = match input {
        serde_json::Value::Array(values) => values.iter().map(count_elements).sum(),
        serde_json::Value::Object(values) => values.values().map(count_elements).sum(),
        _ => 0,
    };
    inner_items + 1
}

pub fn write_value(buffer: &mut [u8], idx: usize, value: BinaryValue) {
    buffer[idx * BinaryValue::SIZE..(idx + 1) * BinaryValue::SIZE]
        .copy_from_slice(value.as_bytes());
}

pub fn fix_parent(buffer: &mut [u8], parent_idx: usize, idx: usize) {
    let value = (idx * BinaryValue::SIZE) as u64;

    buffer[parent_idx * BinaryValue::SIZE..parent_idx * BinaryValue::SIZE + 8]
        .copy_from_slice(&value.to_le_bytes());
}
