pub struct Tone {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub gray: f64,
}

impl Tone {

    pub fn default() -> Tone {
        Tone {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            gray: 0.0,
        }
    }

}