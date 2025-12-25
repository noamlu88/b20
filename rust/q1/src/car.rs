pub struct Car {
    pub color: String,
    pub model: String,
    pub year: u16,
}
impl Car {
    pub fn new(color: String, model: String, year: u16) -> Self {
        Car { color, model, year }
    }

    pub fn print_car(&self) {
        println!("Car Model: {}, Color: {}, Year: {}", self.model, self.color, self.year);
    }
    pub fn change_color(&mut self, new_color: String) {
        self.color = new_color;
    }
}