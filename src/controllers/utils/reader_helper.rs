use std::fs::File;
use std::sync::Arc;
use crate::controllers::utils::object::Object;
use crate::controllers::utils::color::Color;
use crate::controllers::utils::tone::Tone;
use crate::controllers::utils::table::Table;


#[derive(Clone)]
pub enum ReturnTypes {
    Null(Arc<Option<()>>),
    Bool(Arc<bool>),
    Float(Arc<f64>),
    Int(Arc<i32>),
    String(Arc<String>),
    Array(Arc<Vec<Arc<ReturnTypes>>>),
    Hash(Arc<std::collections::HashMap<i32,Arc<ReturnTypes>>>),
    Object(Arc<Object>),
    Symbol(Arc<Vec<u8>>),
    Link(Arc<usize>),
    Symlink(Arc<usize>),
    Color(Arc<Color>),
    Tone(Arc<Tone>),
    Table(Arc<Table>),
}



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
}
