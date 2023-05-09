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
    pub fn clone(table: Table) -> Table {
        Table {
            x_size: table.x_size,
            y_size: table.y_size,
            z_size: table.z_size,
            data: table.data.clone(),
        }
    }
}