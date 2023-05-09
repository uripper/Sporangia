use std::collections::HashMap;
use std::sync::Arc;
use crate::controllers::utils::object::Object;
use crate::controllers::utils::color::Color;
use crate::controllers::utils::tone::Tone;
use crate::controllers::utils::table::Table;
use crate::controllers::utils::pure_object::PureObject;


#[derive(Clone)]
pub enum ReturnTypes<'a> {
    Null(Arc<Option<()>>),
    Bool(Arc<bool>),
    Float(Arc<f64>),
    Int(Arc<i32>),
    String(Arc<String>),
    Array(Arc<Vec<Arc<ReturnTypes<'a>>>>),
    Hash(Arc<HashMap<i32,Arc<ReturnTypes<'a>>>>),
    Object(Arc<Object>),
    Symbol(Arc<Vec<u8>>),
    Link(Arc<usize>),
    Symlink(Arc<usize>),
    Color(Arc<Color>),
    Tone(Arc<Tone>),
    Table(Arc<&'a Table>),
    PureObject(Arc<PureObject>),
}

