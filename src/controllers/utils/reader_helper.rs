use std::any::Any;
use std::fs::File;
use std::sync::Arc;

pub struct Reader {
    pub file: File,
    pub object_cache: Vec<Arc<dyn Any>>,
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
}
