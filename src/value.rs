use std::fmt::Display;

macro_rules! bool_val {
    ($value: expr) => {
        Value::Bool($value)
    };
}

macro_rules! nil_val {
    () => {
        Value::Nil
    };
}

macro_rules! number_val {
    ($value: expr) => {
        Value::Number($value)
    };
}

macro_rules! as_bool {
    ($value: expr) => {
        {
            if let Value::Bool(x) = $value {
                x
            } else {
                panic!("Not bool.")
            }
        }
    };
}

macro_rules! as_number {
    ($value: expr) => {
        {
            if let Value::Number(x) = $value {
                x
            } else {
                panic!("Not number.")
            }
        }
    };
}

macro_rules! is_falsey {
    ($value: expr) => {
        if let Value::Nil | Value::Bool(false) = $value {
            true
        } else {
            false
        }
    };
}

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(x) => write!(f, "{x}"),
            Self::Number(x) => write!(f, "{x}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

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


    pub fn get(&self, seq: usize) -> Value {
        self.values[seq]
    }
}