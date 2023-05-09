use byteorder::{ReadBytesExt, LE};
use std::any::Any;
use std::any::TypeId;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use crate::controllers::utils::color::Color;
use crate::controllers::utils::object::Object;
use crate::controllers::utils::pure_object::PureObject;
use crate::controllers::utils::reader_helper::ReturnTypes;
use crate::controllers::utils::table::Table;
use crate::controllers::utils::tone::Tone;

pub struct Reader {
    pub file: File,
    pub object_cache: Vec<Arc<ReturnTypes>>,
    pub symbol_cache: Vec<Vec<u8>>,
}

impl Reader {
    pub fn new(file: File) -> Self {
        Reader {
            file,
            object_cache: Vec::new(),
            symbol_cache: Vec::new(),
        }
    }
    pub fn parse(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        let mut byte = [0u8];
        self.file.read_exact(&mut byte)?;
        let type_char = byte[0] as char;

        match type_char {
            '@' => Ok(self.parse_link()?),
            'I' => self.parse_ivar(),
            '0' => {
                self.object_cache
                    .push(Arc::new(ReturnTypes::Null(Arc::new(None))));
                Ok(())
            }
            'T' => {
                self.object_cache
                    .push(Arc::new(ReturnTypes::Bool(Arc::new(true))));
                Ok(())
            }
            'F' => {
                self.object_cache
                    .push(Arc::new(ReturnTypes::Bool(Arc::new(false))));
                Ok(())
            }
            'i' => self.parse_fixnum(),
            'f' => self.parse_float(),
            '"' => self.parse_string(),
            '{' => self.parse_hash(),
            'u' => self.parse_userdef(),
            'o' => self.parse_object(),
            ':' => self.parse_symbol(),
            ';' => self.parse_symlink(),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown Value: {}", type_char),
            ))),
        }
    }

    fn parse_link(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let index = self.read_fixnum()?;
        if index as usize >= self.object_cache.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Link index out of range",
            )));
        }
        self.object_cache
            .push(Arc::new(ReturnTypes::Link(Arc::new(index as usize))));
        Ok(())
    }
    fn parse_ivar(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.parse()?; // Parse and discard the name, since we're not using it

        let length = self.read_fixnum()?;

        for _ in 0..length {
            let key = {
                let key_any = self.parse()?;
                if key_any.type_id() != TypeId::of::<Vec<u8>>() {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "IVar key not symbol",
                    )));
                }
                Arc::try_unwrap(key_any.into()).unwrap_or_else(|a| (*a).clone())
            };

            self.parse()?; // Parse and discard the value, since we're not using it
        }

        Ok(())
    }

    fn parse_fixnum(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let num = self.read_fixnum()?;
        self.object_cache
            .push(Arc::new(ReturnTypes::Int(Arc::new(num))));
        Ok(())
    }
    fn read_fixnum(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let mut byte_buffer = [0u8];
        self.file.read_exact(&mut byte_buffer)?;
        let mut byte_count = byte_buffer[0] as i8;

        let mut result;
        if byte_count == 0 {
            return Ok(0);
        } else if byte_count > 0 {
            if 4 < byte_count && byte_count < 128 {
                return Ok((byte_count - 5) as i32);
            }
            if byte_count as usize > std::mem::size_of::<i32>() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Fixnum too big: {}", byte_count),
                )));
            }

            result = 0;
            for _ in 0..4 {
                let mut byte = [0u8];
                self.file.read_exact(&mut byte)?;
                if byte_count > 0 {
                    result = (byte[0] as u32 >> 24) | (result >> 8);
                    byte_count -= 1;
                } else {
                    result = result >> 8;
                }
            }
        } else {
            if -129 < byte_count && byte_count < -4 {
                return Ok((byte_count + 5) as i32);
            }
            byte_count = -byte_count;
            if byte_count as usize > std::mem::size_of::<i32>() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Fixnum too big: {}", byte_count),
                )));
            }

            result = !0;
            const MASK: u32 = !(0xff << 24);
            for _ in 0..4 {
                let mut byte = [0u8];
                self.file.read_exact(&mut byte)?;
                if byte_count > 0 {
                    result = (byte[0] as u32 >> 24) | ((result >> 8) & MASK);
                    byte_count -= 1;
                } else {
                    result = (result >> 8) & MASK;
                }
            }
        }

        Ok(result as i32)
    }

    fn parse_float(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let length = self.read_fixnum()?;
        let mut buf = vec![0u8; length as usize];
        self.file.read_exact(&mut buf)?;

        let str_val = String::from_utf8_lossy(&buf);
        let v = match str_val.as_ref() {
            "nan" => std::f64::NAN,
            "inf" => std::f64::INFINITY,
            "-inf" => std::f64::NEG_INFINITY,
            _ => str_val.parse::<f64>().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse float")
            })?,
        };

        let float_obj = Arc::new(ReturnTypes::Float(Arc::new(v)));
        self.object_cache.push(float_obj);
        Ok(())
    }

    fn parse_string(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let length = self.read_fixnum()? as usize;
        let mut buf = vec![0u8; length];
        self.file.read_exact(&mut buf)?;

        let str_val = String::from_utf8(buf)?;
        let string_obj = Arc::new(ReturnTypes::String(Arc::new(str_val)));
        self.object_cache.push(string_obj);
        Ok(())
    }


    fn try_fixnum(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        if self.read_fixnum()? != 0 {
            println!("Not Fixnum");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Not Fixnum",
            )));
        } else {
            self.read_fixnum()
        }
    }

    fn parse_hash(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let length = self.read_fixnum()?;
        let mut map: std::collections::HashMap<i32, Arc<ReturnTypes>> =
            std::collections::HashMap::new();

        for _ in 0..length {
            let key_any = self.try_fixnum()?;
            let key = key_any;
            let value = self.parse()?;
            map.insert(key, Arc::new(value));
        }

        let hash_obj = Arc::new(ReturnTypes::Hash(Arc::new(map)));
        self.object_cache.push(hash_obj);
        Ok(())
    }

    fn parse_userdef(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pname = self.parse()?;
        let name = match pname {
            ReturnTypes::Symbol(s) => String::from_utf8(s.to_vec())?,
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "UserDef name not Symbol",
                )))
            }
        };

        let size = self.read_fixnum()?;
        match name.as_str() {
            "Color" => self.parse_color(),
            "Table" => self.parse_table(),
            "Tone" => self.parse_tone(),
            "Object" => self.parse_object(),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown user-defined type: {}", name),
            ))),
        }
    }

    fn parse_color(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut color = Color::default();
        color.red = self.file.read_f64::<LE>()?;
        color.green = self.file.read_f64::<LE>()?;
        color.blue = self.file.read_f64::<LE>()?;
        color.alpha = self.file.read_f64::<LE>()?;

        let color_obj = Arc::new(ReturnTypes::Color(Arc::new(color)));
        self.object_cache.push(color_obj);
        Ok(())
    }

    fn parse_table(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut table = Table::default();
        table.x_size = self.file.read_i32::<LE>()?;
        table.y_size = self.file.read_i32::<LE>()?;
        table.z_size = self.file.read_i32::<LE>()?;
        let table_size = self.file.read_i32::<LE>()?;

        table.data.resize(table_size as usize, 0);
        for i in 0..table_size {
            table.data[i as usize] = self.file.read_i16::<LE>()?;
        }

        let table_obj = Arc::new(ReturnTypes::Table(Arc::new(table)));
        self.object_cache.push(table_obj);
        Ok(())
    }

    fn parse_tone(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let mut tone = Tone::default();
        tone.red = self.file.read_f64::<LE>()?;
        tone.green = self.file.read_f64::<LE>()?;
        tone.blue = self.file.read_f64::<LE>()?;
        tone.gray = self.file.read_f64::<LE>()?;

        let tone_obj = Arc::new(tone);
        self.object_cache
            .push(Arc::new(ReturnTypes::Tone(tone_obj.clone())));
        Ok(ReturnTypes::Tone(tone_obj))
    }

fn parse_array(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let len = self.read_fixnum()?;
    let mut arr: Vec<Arc<ReturnTypes>> = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let elem_index = self.parse()?;
        let elem = self.object_cache[elem_index].clone();
        arr.push(elem);
    }

    let array_obj = Arc::new(ReturnTypes::Array(Arc::new(arr)));
    self.object_cache.push(array_obj);
    Ok(())
}
    fn parse_object(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let name = self.parse()?;
        let name = if let ReturnTypes::Symbol(name) = name {
            String::from_utf8(Arc::try_unwrap(name).unwrap_or_else(|a| (*a).clone()))?
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Object name not Symbol",
            )));
        };

        let length = self.read_fixnum()? as usize;
        let mut o = PureObject { name, list: vec![] };
        o.list.reserve(length);
        self.object_cache
            .push(ReturnTypes::Object(Arc::new(Object::clone())).into());

        for _ in 0..length {
            let key = {
                let key_any = self.parse()?;
                if let ReturnTypes::Symbol(key) = key_any {
                    String::from_utf8(Arc::try_unwrap(key).unwrap_or_else(|a| (*a).clone()))?
                } else {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Object key not symbol",
                    )));
                }
            };

            if key.chars().nth(0) != Some('@') {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Object Key not instance variable name",
                )));
            }

            let value = self.parse()?;
            o.list.push(Box::new((key, value)));
        }

        Ok(ReturnTypes::PureObject(Arc::new(o)))
    }

    fn parse_symbol(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let length = self.read_fixnum()? as usize;
        let mut buf = vec![0u8; length];
        self.file.read_exact(&mut buf)?;

        let symbol = Arc::new(buf);
        self.symbol_cache.push(symbol.to_vec());
        Ok(ReturnTypes::Symbol(symbol))
    }

    fn parse_symlink(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let index = self.read_fixnum()? as usize;
        if let Some(symbol) = self.symbol_cache.get(index) {
            Ok(ReturnTypes::Symlink(Arc::new(index)))
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Symlink index out of range",
            )))
        }
    }
}
