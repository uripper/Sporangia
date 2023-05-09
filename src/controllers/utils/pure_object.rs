use std::any::Any;

pub struct PureObject {
    pub name: String,
    pub list: Vec<Box<dyn Any>>,
}

impl PureObject {

    pub fn default() -> PureObject {
        PureObject {
            name: String::new(),
            list: Vec::new(),
        }
    }

    pub fn clone() -> PureObject {
        PureObject {
            name: String::new(),
            list: Vec::new(),
        }
    }
}