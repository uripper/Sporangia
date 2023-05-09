use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ParsedValue {
    Link(i32),
    IVar(String),
    Nil,
    Bool(bool),
    Fixnum(i32),
    Float(f64),
    String(String),
    Array(Vec<ParsedValue>),
    Hash(std::collections::HashMap<i32, ParsedValue>),
    UserDef(String),
    Object(String, std::collections::HashMap<String, ParsedValue>),
    Symbol(Vec<u8>),
}

fn parse_value(input: &[u8]) -> Result<ParsedValue, Box<dyn std::error::Error>> {
    let value = bincode::deserialize(input)?;
    Ok(value)
}

fn parse_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read(file_path)?;

    let mut remaining_data = &data[..];
    let mut object_cache = Vec::new();

    while !remaining_data.is_empty() {
        let parsed_value = parse_value(remaining_data)?;
        remaining_data = &remaining_data[parsed_value_size(&parsed_value)..];

        match parsed_value {
            ParsedValue::Link(index) => {
                let index = index - 1;

                if index < 0 || index as usize >= object_cache.len() {
                    return Err("Invalid link index".into());
                }

                let object = object_cache[index as usize].clone();
                object_cache.push(object);
            }
            _ => {
                object_cache.push(parsed_value);
            }
        }

        println!("Parsed value: {:?}", parsed_value);
    }

    Ok(())
}

fn parsed_value_size(parsed_value: &ParsedValue) -> usize {
    bincode::serialized_size(parsed_value).unwrap() as usize
}

