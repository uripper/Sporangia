pub struct Table {
    pub x_size: i32,
    pub y_size: i32,
    pub z_size: i32,
    pub data: Vec<i16>,
}


impl Table {

    pub fn default() -> Table {
        Table {
            x_size: 0,
            y_size: 0,
            z_size: 0,
            data: Vec::new(),
        }
    }
}