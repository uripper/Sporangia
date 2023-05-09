use std::collections::HashMap;
use std::any::Any;

pub struct Object {
    pub name: String,
    pub list: HashMap<String, Box<dyn Any>>,
}
