use std::any::Any;
use std::any::TypeId;
use std::io::{Read, Seek, SeekFrom};
use std::slice;
use std::sync::Arc;

use crate::controllers::utils::color::Color;
use crate::controllers::utils::object::Object;
use crate::controllers::utils::reader_helper::Reader;
use crate::controllers::utils::table::Table;
use crate::controllers::utils::tone::Tone;

#[derive(Clone)]
enum ReturnTypes {
    Null(Arc<Option<()>>),
    Bool(Arc<bool>),
    Float(Arc<f64>),
    Int(Arc<i32>),
    String(Arc<String>),
    Array(Arc<Vec<Arc<dyn Any>>>),
    Hash(Arc<Vec<(Arc<dyn Any>, Arc<dyn Any>)>>),
    Object(Arc<Object>),
    Symbol(Arc<Vec<u8>>),
    Link(Arc<usize>),
    Symlink(Arc<usize>),
    Color(Arc<Color>),
    Tone(Arc<Tone>),
    Table(Arc<Table>),
}

impl Reader {
    pub fn parse(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let mut byte = [0u8];
        self.file.read_exact(&mut byte)?;
        let type_char = byte[0] as char;

        match type_char {
            '@' => self.parse_link(),
            'I' => Ok(ReturnTypes::Object(self.parse_ivar()?)),
            '0' => Ok(ReturnTypes::Null(Arc::new(None))),
            'T' => Ok(ReturnTypes::Bool(Arc::new(true))),
            'F' => Ok(ReturnTypes::Bool(Arc::new(false))),
            'i' => self.parse_fixnum(),
            'f' => Ok(ReturnTypes::Float(self.parse_float()?)),
            '"' => Ok(ReturnTypes::String(self.parse_string()?)),
            '[' => self.parse_array(),
            '{' => Ok(ReturnTypes::Hash(self.parse_hash()?)),
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

    fn parse_link(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let index = self.read_fixnum()?;
        if index as usize >= self.object_cache.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Link index out of range",
            )));
        }
        Ok(ReturnTypes::Link(Arc::new(index as usize)))
    }

    fn parse_ivar(&mut self) -> Result<Arc<Object>, Box<dyn std::error::Error>> {
        let name = self.parse()?;
        if name.type_id() != TypeId::of::<String>() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unsupported IVar Type",
            )));
        }

        let length = self.read_fixnum()?;
        self.object_cache.push(name.clone());

        for _ in 0..length {
            let key = {
                let key_any = self.parse()?;
                if key_any.type_id() != TypeId::of::<Vec<u8>>() {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "IVar key not symbol",
                    )));
                }
                Arc::try_unwrap(key_any).unwrap_or_else(|a| (*a).clone())
            };

            let _value = self.parse();
        }

        Ok(name)
    }

    fn parse_fixnum(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let num = self.read_fixnum()?;
        Ok(ReturnTypes::Int(Arc::new(num)))
    }

    fn read_fixnum(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let mut c = [0u8];
        self.file.read_exact(&mut c)?;
        let c = c[0] as i8;

        let x;
        if c == 0 {
            return Ok(0);
        } else if c > 0 {
            if 4 < c && c < 128 {
                return Ok((c - 5) as i32);
            }
            if c as usize > std::mem::size_of::<i32>() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Fixnum too big: {}", c),
                )));
            }

            x = 0;
            for _ in 0..4 {
                let mut b = [0u8];
                self.file.read_exact(&mut b)?;
                x = (if c > 0 {
                    b[0] as u32
                } << 24)
                    | (x >> 8);
                c -= 1;
            }
        } else {
            if -129 < c && c < -4 {
                return Ok((c + 5) as i32);
            }
            c = -c;
            if c as usize > std::mem::size_of::<i32>() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Fixnum too big: {}", c),
                )));
            }

            x = !0;
            const MASK: u32 = !(0xff << 24);
            for _ in 0..4 {
                let mut b = [0u8];
                self.file.read_exact(&mut b)?;
                x = (if c > 0 {
                    b[0] as u32
                } << 24)
                    | ((x >> 8) & MASK);
                c -= 1;
            }
        }

        Ok(x as i32)
    }

    fn parse_float(&mut self) -> Result<Arc<f64>, Box<dyn std::error::Error>> {
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

        let float_obj = Arc::new(v);
        self.object_cache.push(float_obj.clone());
        Ok(float_obj)
    }

    fn parse_string(&mut self) -> Result<Arc<String>, Box<dyn std::error::Error>> {
        let length = self.read_fixnum()? as usize;
        let mut buf = vec![0u8; length];
        self.file.read_exact(&mut buf)?;

        let str_val = String::from_utf8(buf)?;
        let string_obj = Arc::new(str_val);
        self.object_cache.push(string_obj.clone());
        Ok(string_obj)
    }

    fn parse_array(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let len = self.read_fixnum()?;
        let mut arr: Vec<Arc<dyn Any>> = Vec::with_capacity(len as usize);
        self.object_cache.push(Arc::new(arr.clone()));

        for _ in 0..len {
            let elem = self.parse()?;
            arr.push(elem);
        }

        let array_obj = Arc::new(arr);
        self.object_cache.push(array_obj.clone());
        Ok(array_obj)
    }

    fn parse_hash(&mut self) -> Result<Arc<dyn Any>, Box<dyn std::error::Error>> {
        let length = self.read_fixnum()?;
        let mut map: std::collections::HashMap<i32, Arc<dyn Any>> =
            std::collections::HashMap::new();
        self.object_cache.push(Arc::new(map.clone()));

        for _ in 0..length {
            let key_any = self.parse()?;
            if key_any.type_id() != TypeId::of::<i32>() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Hash key not Fixnum",
                )));
            }
            let key = Arc::try_unwrap(key_any).unwrap_or_else(|a| (*a).clone()) as i32;

            let value = self.parse()?;
            map.insert(key, value);
        }

        let hash_obj = Arc::new(map);
        self.object_cache.push(hash_obj.clone());
        Ok(hash_obj)
    }

    fn parse_userdef(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let pname = self.parse()?;
        let name = if pname.type_id() == TypeId::of::<Vec<u8>>() {
            String::from_utf8(Arc::try_unwrap(pname).unwrap_or_else(|a| (*a).clone()))?
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "UserDef name not Symbol",
            )));
        };

        let size = self.read_fixnum()?;
        match determined_type {
            "Color" => self.parse_color().map(ReturnTypes::Color),
            "Table" => self.parse_table().map(ReturnTypes::Table),
            "Tone" => self.parse_tone().map(ReturnTypes::Tone),
            "Object" => self.parse_object().map(ReturnTypes::Object),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown user-defined type: {}", determined_type),
            ))),
        }
        if name == "Color" {
            let mut color = Color::default();
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut color.red) as *mut f64 as *mut u8,
                8,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut color.green) as *mut f64 as *mut u8,
                8,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut color.blue) as *mut f64 as *mut u8,
                8,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut color.alpha) as *mut f64 as *mut u8,
                8,
            ))?;

            let color_obj = Arc::new(color);
            self.object_cache.push(color_obj.clone());
            Ok(color_obj)
        } else if name == "Table" {
            let mut table = Table::default();
            self.file.seek(SeekFrom::Current(4))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut table.x_size) as *mut i32 as *mut u8,
                4,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut table.y_size) as *mut i32 as *mut u8,
                4,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut table.z_size) as *mut i32 as *mut u8,
                4,
            ))?;

            let mut table_size = 0i32;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut table_size) as *mut i32 as *mut u8,
                4,
            ))?;

            table.data.resize(table_size as usize);
            for i in 0..table_size {
                let mut data = 0i16;
                self.file.read_exact(slice::from_raw_parts_mut(
                    (&mut data) as *mut i16 as *mut u8,
                    2,
                ))?;
                table.data[i as usize] = data;
            }

            let table_obj = Arc::new(table);
            self.object_cache.push(table_obj.clone());
            Ok(table_obj)
        } else if name == "Tone" {
            let mut tone = Tone::default();
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut tone.red) as *mut f64 as *mut u8,
                8,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut tone.green) as *mut f64 as *mut u8,
                8,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut tone.blue) as *mut f64 as *mut u8,
                8,
            ))?;
            self.file.read_exact(slice::from_raw_parts_mut(
                (&mut tone.gray) as *mut f64 as *mut u8,
                8,
            ))?;

            let tone_obj = Arc::new(tone);
            self.object_cache.push(tone_obj.clone());
            Ok(tone_obj)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported user defined class: {}", name),
            )))
        }
    }

    fn parse_object(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let name = self.parse()?;
        let name = if name.type_id() == TypeId::of::<Vec<u8>>() {
            String::from_utf8(Arc::try_unwrap(name).unwrap_or_else(|a| (*a).clone()))?
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Object name not Symbol",
            )));
        };

        let length = self.read_fixnum()?;
        let mut o = Object { name, list: vec![] };
        o.list.reserve(length as usize);
        self.object_cache.push(Arc::new(o.clone()));

        for _ in 0..length {
            let key = {
                let key_any = self.parse()?;
                if key_any.type_id() != TypeId::of::<Vec<u8>>() {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Object key not symbol",
                    )));
                }
                String::from_utf8(Arc::try_unwrap(key_any).unwrap_or_else(|a| (*a).clone()))?
            };

            if key.chars().nth(0) != Some('@') {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Object Key not instance variable name",
                )));
            }

            let value = self.parse()?;
            o.list.push((key, value));
        }

        Ok(Arc::new(o))
    }

    fn parse_symbol(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let length = self.read_fixnum()? as usize;
        let mut buf = vec![0u8; length];
        self.file.read_exact(&mut buf)?;

        let symbol = Arc::new(buf);
        self.symbol_cache.push(symbol.clone());
        Ok(symbol)
    }

    fn parse_symlink(&mut self) -> Result<ReturnTypes, Box<dyn std::error::Error>> {
        let index = self.read_fixnum()? as usize;
        if let Some(symbol) = self.symbol_cache.get(index) {
            Ok(symbol.clone())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Symlink index out of range",
            )))
        }
    }
}
