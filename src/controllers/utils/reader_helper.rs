use std::sync::Arc;
use crate::controllers::utils::object::Object;
use crate::controllers::utils::color::Color;
use crate::controllers::utils::tone::Tone;
use crate::controllers::utils::table::Table;
use crate::controllers::utils::pure_object::PureObject;


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
    PureObject(Arc<PureObject>),
}
