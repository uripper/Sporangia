use std::collections::HashMap;
use std::any::Any;

pub struct Object {
    pub name: String,
    pub list: HashMap<String, Box<dyn Any>>,
}

impl Object {

    pub fn default() -> Object {
        Object {
            name: String::new(),
            list: HashMap::new(),
        }
    }

    pub fn clone() -> Object {
        Object {
            name: String::new(),
            list: HashMap::new(),
        }
    }
}