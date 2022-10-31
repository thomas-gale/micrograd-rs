use std::fmt;
use std::ops;

pub struct Value<T> {
    pub data: T,
}

impl<T: fmt::Display> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value (data: {}, ...)", self.data)
    }
}

impl<T: fmt::Display + ops::Add> Value<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T: fmt::Display + ops::Add<Output = T>> ops::Add for Value<T> {
    type Output = Value<T>;
    fn add(self, rhs: Self) -> Self::Output {
        println!("Adding {} to {}", self, rhs);
        Value::new(self.data + rhs.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_display_value() {
        let v1 = Value::new(1i32);
        println!("{}", v1);
    }

    #[test]
    fn can_add_value() {
        let v1 = Value::new(1i32);
        let v2 = Value::new(2i32);
        let v3 = v1 + v2;
        println!("{}", v3);
    }
}
