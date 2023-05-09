use nom::{number::streaming::be_u8, IResult, number::streaming::be_i32};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
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

fn parser_caller(input: &[u8]) -> IResult<&[u8], ParsedValue> {
    let (input, type_byte) = be_u8(input)?;

    match type_byte {
        b'@' => unimplemented!("parse_link"),
        b'I' => unimplemented!("parse_ivar"),
        b'0' => unimplemented!("parse_nil"),
        b'T' => unimplemented!("parse_true"),
        b'F' => unimplemented!("parse_false"),
        b'i' => unimplemented!("parse_fixnum"),
        b'f' => unimplemented!("parse_float"),
        b'l' => unimplemented!("parse_bignum"),
        b'"' => unimplemented!("parse_string"),
        b'[' => unimplemented!("parse_array"),
        b'{' => unimplemented!("parse_hash"),
        b'u' => unimplemented!("parse_userdef"),
        b'o' => unimplemented!("parse_object"),
        b':' => unimplemented!("parse_symbol"),
        b';' => unimplemented!("parse_symlink"),
        b'e' => unimplemented!("parse_extended"),
        b'C' => unimplemented!("parse_uclass"),
        b'S' => unimplemented!("parse_struct"),
        b'U' => unimplemented!("parse_usermarshal"),
        b'd' => unimplemented!("parse_data"),
        b'M' => unimplemented!("parse_moduleold"),
        b'c' => unimplemented!("parse_class"),
        b'm' => unimplemented!("parse_module"),
        _ => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Eof)))
,
    }
}

fn parser_scanner(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file into a byte vector
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Call the parser_caller function
    let result = parser_caller(&data);

    match result {
        Ok((remaining_data, parsed_value)) => {
            // Handle the parsed value
            println!("Parsed value: {:?}", parsed_value);

            // If there's any data left, you could continue parsing here
            if !remaining_data.is_empty() {
                println!("There's more data to parse!");
            }
        }
        Err(err) => {
            // Handle the error
            println!("An error occurred: {:?}", err);
        }
    }

    Ok(())
}

fn parse_link<'a>(input: &'a [u8], object_cache: &'a Vec<ParsedValue>) -> IResult<&'a [u8], ParsedValue> {
    let (input, index) = be_i32(input)?;

    // Subtract 1 from the index because the C++ code does so
    let index = index - 1;

    // Check if the index is valid
    if index < 0 || index as usize >= object_cache.len() {
        return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Eof)));
    }

    // Retrieve the object from the cache
    let object = object_cache[index as usize].clone();

    Ok((input, object))
}

use nom::{number::streaming::be_i8};

fn read_fixnum(input: &[u8]) -> IResult<&[u8], i32> {
    let (input, c) = be_i8(input)?;

    if c == 0 {
        return Ok((input, 0));
    }

    if c > 0 {
        if c <= 127 {
            return Ok((input, c as i32 - 5));
        }

        let (input, x) = be_i32(input)?;

        return Ok((input, x));
    } else {
        if c >= -128 {
            return Ok((input, c as i32 + 5));
        }

        let (input, x) = be_i32(input)?;

        return Ok((input, -x));
    }
}

fn parse_fixnum(input: &[u8]) -> IResult<&[u8], ParsedValue> {
    let (input, value) = read_fixnum(input)?;
    Ok((input, ParsedValue::Fixnum(value)))
}

