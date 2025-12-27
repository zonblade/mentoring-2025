struct Temp {
    degrees_c : f64
}

impl Temp {
    fn freezing() -> Self {
        Self { degrees_c: 0.0}
    }
    fn boiling() -> Self {
        Self { degrees_c: 100.0}
    }
    fn show_temp(&self) {
        println!("{:?} degrees c", self.degrees_c)
    }
}
