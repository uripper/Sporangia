use std::fs::File;
use std::io::{self, BufReader, Read};
use std::any::Any;
use std::collections::HashMap;

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
            b'@' => self.handle_link(),
            b'I' => self.handle_ivar(),
            b'e' => self.handle_extended(),
            b'C' => self.handle_class(),
            b'0' => self.handle_nil(),
            b'T' => self.handle_true(),
            b'F' => self.handle_false(),
            b'i' => self.read_fixnum(),
            b'f' => self.handle_float(),
            b'l' => self.handle_bignum(),
            b'"' => self.handle_string(),
            b'[' => self.handle_array(),
            b'/' => self.handle_regexp(),
            b'{' => self.handle_hash(),
            }
            b'}' => self.handle_hash_end(),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown type identifier")),
        }
    }

    fn handle_link(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.object_cache.len() as u32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid object index"));
        }
        Ok(self.object_cache[index as usize].clone())
    }

    fn handle_ivar(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.object_cache.len() as u32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid object index"));
        }
        let object = self.object_cache[index as usize].clone();
        let object = object.downcast::<HashMap<Vec<u8>, Box<dyn Any>>>().unwrap();
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.symbol_cache.len() as u32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid symbol index"));
        }
        let symbol = self.symbol_cache[index as usize].clone();
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let value = self.parse()?;
        object.insert(symbol, value);
        Ok(Box::new(object))
    }

    fn handle_extended(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 1];
        self.file.read_exact(&mut buf)?;
        let type_id = buf[0];
        match type_id {
            b'c' => self.handle_cache(),
            b's' => self.handle_symbol(),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown extended type identifier")),
        }
    }

    fn handle_class(&mut self) -> io::Result<Box<dyn Any>> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        let index = u32::from_be_bytes(buf);
        if index >= self.symbol_cache.len() as u32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid symbol index"));
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

    fn handle_nil(&mut self) -> io::Result<Box<dyn Any>> {
        Ok(Box::new(()))
    }

    fn handle_true(&mut self) -> io::Result<Box<dyn Any>> {
        Ok(Box::new(true))
    }

    fn handle_false(&mut self) -> io::Result<Box<dyn Any>> {
        Ok(Box::new(false))
    }

    fn read_fixnum(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
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
        let mut buf = [0; 8];
        self.file.read_exact(&mut buf)?;
        let float = f64::from_be_bytes(buf);
        Ok(Box::new(float))
    }
}
