use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Clone)]
struct Object {
    class: String,
    list: HashMap<String, Box<dyn Any>>,
}

#[derive(Clone)]
struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

#[derive(Clone)]
struct Table {
    name: String,
    color: Color,
    width: i32,
    height: i32,
    data: Vec<Vec<String>>,
}

#[derive(Clone)]
struct Tone {
    red: f64,
    green: f64,
    blue: f64,
    grey: f64,
}

struct Reader {
    file: BufReader<File>,
    object_cache: Vec<Box<dyn Any>>,
    symbol_cache: Vec<Vec<u8>>,
}

impl Reader {
    fn new(file: File) -> Reader {
        Reader {
            file: BufReader::new(file),
            object_cache: Vec::new(),
            symbol_cache: Vec::new(),
        }
    }

    fn parse(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 1];
        self.file.read_exact(&mut buf)?;
        let type_id = buf[0];
        match type_id {
            b'[' => self.handle_array(),
            b'l' => self.handle_bignum(),
            b'C' => self.handle_class(),
            b'd' => self.handle_data(),
            b'e' => self.handle_extended(),
            b'F' => self.handle_false(),
            b'i' => self.handle_fixnum(),
            b'f' => self.handle_float(),
            b'{' => self.handle_hash(),
            b'}' => self.handle_hash_def(),
            b'I' => self.handle_ivar(),
            b'@' => self.handle_link(),
            b'm' => self.handle_module(),
            b'0' => self.handle_nil(),
            b'/' => self.handle_regexp(),
            b'"' => self.handle_string(),
            b'S' => self.handle_struct(),
            b':' => self.handle_symbol(),
            b';' => self.handle_symlink(),
            b'T' => self.handle_true(),
            b'u' => self.handle_user_defined(),
            b'U' => self.handle_user_marshall(),

            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown type identifier",
            )),
        }
    }

    fn handle_array(&mut self) -> io::Result<Box<dyn Any>> {
        let length = self.read_fixnum()? as usize;
        let mut array: Vec<Box<dyn Any>> = Vec::with_capacity(length);
        for _ in 0..length {
            let value = self.parse()?;
            array.push(value);
        }
        let array_box = Box::new(array);
        self.object_cache.push(array_box.clone());
        Ok(array_box)
    }

    fn handle_bignum(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        let mut result = 0;
        for byte in buf {
            result = result << 8 | byte as i64;
        }
        Ok(Box::new(result))
    }

    fn handle_class(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.symbol_cache.len() as u32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid symbol index",
            ));
        }
        let symbol = self.symbol_cache[index as usize].clone();
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let superclass = self.parse()?;
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let instance_variables = self.parse()?;
        let mut class = HashMap::new();
        class.insert(b"symbol".to_vec(), Box::new(symbol));
        class.insert(b"superclass".to_vec(), superclass);
        class.insert(b"instance_variables".to_vec(), instance_variables);
        Ok(Box::new(class))
    }

    fn handle_color(&mut self) -> io::Result<Box<dyn Any>> {
        let mut color = Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        };
        self.file.read_exact(bytemuck::cast_slice_mut(&mut color))?;
        Ok(Box::new(color))
    }

    fn handle_data(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(Box::new(buf))
    }

    fn handle_extended(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 1];
        self.file.read_exact(&mut buf)?;
        let type_id = buf[0];
        match type_id {
            b'c' => self.handle_cache(),
            b's' => self.handle_symbol(),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown extended type identifier",
            )),
        }
    }

    fn handle_false(&mut self) -> io::Result<Box<dyn Any>> {
        Ok(Box::new(false))
    }

    fn handle_fixnum(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        // Read the first byte from the file
        let mut first_byte = [0u8; 1];
        self.file.read_exact(&mut first_byte)?;
        let first_byte = first_byte[0] as i32;

        // If the first byte is 0, the Fixnum is 0
        if first_byte == 0 {
            return Ok(0);
        }

        let mut result: i32;
        if first_byte > 0 {
            // If the first byte is between 4 and 128, it is the Fixnum minus 5
            if 4 < first_byte && first_byte < 128 {
                return Ok(first_byte - 5);
            }
            // If the first byte is greater than the size of an i32, it's too big
            if first_byte as usize > std::mem::size_of::<i32>() {
                return Err("Fixnum too big".into());
            }

            // Initialize the result to 0
            result = 0;
            for i in 0..4 {
                let mut byte = [0u8; 1];
                self.file.read_exact(&mut byte)?;
                let byte = byte[0] as i32;
                // Shift and combine bytes to form the Fixnum
                result = (if i < first_byte { byte << 24 } else { 0 }) | (result >> 8);
            }
        } else {
            // If the first byte is between -4 and -129, it is the Fixnum plus 5
            if -129 < first_byte && first_byte < -4 {
                return Ok(first_byte + 5);
            }
            let first_byte = -first_byte;
            // If the first byte is greater than the size of an i32, it's too big
            if first_byte as usize > std::mem::size_of::<i32>() {
                return Err("Fixnum too big".into());
            }

            // Initialize the result to -1
            result = -1;
            let mask = !(0xff << 24);
            for i in 0..4 {
                let mut byte = [0u8; 1];
                self.file.read_exact(&mut byte)?;
                let byte = byte[0] as i32;
                // Shift and combine bytes to form the Fixnum
                result = (if i < first_byte { byte << 24 } else { 0xff }) | ((result >> 8) & mask);
            }
        }

        Ok(result)
    }

    fn handle_float(&mut self) -> io::Result<Box<dyn Any>> {
        let length = self.read_fixnum()? as usize;
        let mut buf = vec![0; length];
        self.file.read_exact(&mut buf)?;
        let s = String::from_utf8_lossy(&buf);
        let float = match s.as_ref() {
            "nan" => std::f64::NAN,
            "inf" => std::f64::INFINITY,
            "-inf" => std::f64::NEG_INFINITY,
            _ => s
                .parse::<f64>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid float"))?,
        };
        Ok(Box::new(float))
    }

    fn handle_hash(&mut self) -> io::Result<Box<dyn Any>> {
        let length = self.read_fixnum()? as usize;

        let mut map: HashMap<i32, Box<dyn Any>> = HashMap::with_capacity(length);

        for _ in 0..length {
            let key = self.parse()?;
            let key = match key.downcast_ref::<i32>() {
                Some(&i) => i,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Hash key not Fixnum",
                    ))
                }
            };

            let value = self.parse()?;

            map.insert(key, value);
        }

        Ok(Box::new(map))
    }

    fn handle_hash_def(&mut self) -> io::Result<Box<dyn Any>> {
        let mut hash = HashMap::new();
        let default = self.parse()?;
        hash.insert(b"default".to_vec(), default);
        Ok(Box::new(hash))
    }

    fn handle_ivar(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.object_cache.len() as u32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid object index",
            ));
        }
        let object = self.object_cache[index as usize].clone();
        let object = object.downcast::<HashMap<Vec<u8>, Box<dyn Any>>>().unwrap();
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.symbol_cache.len() as u32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid symbol index",
            ));
        }
        let symbol = self.symbol_cache[index as usize].clone();
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let value = self.parse()?;
        object.insert(symbol, value);
        Ok(Box::new(object))
    }

    fn handle_link(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.object_cache.len() as u32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid object index",
            ));
        }
        Ok(self.object_cache[index as usize].clone())
    }

    fn handle_module(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(Box::new(buf))
    }

    fn handle_nil(&mut self) -> io::Result<Box<dyn Any>> {
        Ok(Box::new(()))
    }

    fn handle_object(&mut self) -> io::Result<Box<dyn Any>> {
        let name_as_bytes = self.parse()?;
        let name = match name_as_bytes.downcast_ref::<Vec<u8>>() {
            Some(vec) => String::from_utf8_lossy(vec).into_owned(),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Object name not Symbol",
                ))
            }
        };

        let length = self.read_fixnum()? as usize;

        let mut object = Object {
            name,
            list: HashMap::with_capacity(length),
        };

        self.object_cache.push(Box::new(object.clone()));

        for _ in 0..length {
            let key_as_bytes = self.parse()?;
            let key = match key_as_bytes.downcast_ref::<Vec<u8>>() {
                Some(vec) => String::from_utf8_lossy(vec).into_owned(),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Object key not symbol",
                    ))
                }
            };

            if !key.starts_with('@') {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Object Key not instance variable name",
                ));
            }

            let value = self.parse()?;

            object.list.insert(key, value);
        }

        Ok(Box::new(object))
    }

    fn handle_regexp(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let options = u32::from_be_bytes(buf);
        let mut regexp = HashMap::new();
        regexp.insert(b"source".to_vec(), Box::new(buf));
        regexp.insert(b"options".to_vec(), Box::new(options));
        Ok(Box::new(regexp))
    }

    fn handle_string(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(Box::new(buf))
    }

    fn handle_struct(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut struct_ = HashMap::new();
        for _ in 0..length {
            let key = self.parse()?;
            let value = self.parse()?;
            struct_.insert(key, value);
        }
        Ok(Box::new(struct_))
    }

    fn handle_symbol(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(Box::new(buf))
    }

    fn handle_symlink(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(Box::new(buf))
    }

    fn handle_tone(&mut self) -> io::Result<Box<dyn Any>> {
        fn handle_table(&mut self) -> io::Result<Box<dyn Any>> {
            let mut table = Table {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            self.file.read_exact(bytemuck::cast_slice_mut(&mut table))?;
            Ok(Box::new(table))
        }

        let mut tone = Tone {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            gray: 0.0,
        };
        self.file.read_exact(bytemuck::cast_slice_mut(&mut tone))?;
        Ok(Box::new(tone))
    }

    fn handle_true(&mut self) -> io::Result<Box<dyn Any>> {
        Ok(Box::new(true))
    }

    fn handle_user_defined(&mut self) -> io::Result<Box<dyn Any>> {
        let type_name_as_bytes = self.parse()?;
        let type_name = match type_name_as_bytes.downcast_ref::<Vec<u8>>() {
            Some(vec) => String::from_utf8_lossy(vec).into_owned(),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "UserDef name not Symbol",
                ))
            }
        };

        match type_name.as_str() {
            "Color" => self.handle_color(),
            "Table" => self.handle_table(),
            "Tone" => self.handle_tone(),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported user defined class: {}", type_name),
            )),
        }
    }

    fn handle_user_marshall(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(Box::new(buf))
    }
}
