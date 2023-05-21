pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value)
    } 

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn print_value(&self, constant: usize) {
        print!("{}", self.values[constant])
    }
}