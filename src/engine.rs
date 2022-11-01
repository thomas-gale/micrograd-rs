use std::collections::BTreeSet;
use std::{cmp, fmt, ops};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value<T> {
    pub data: T,
    prev: BTreeSet<Value<T>>,
}

impl<T: fmt::Display + fmt::Debug> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value (data: {}, prev: {:?})", self.data, self.prev)
    }
}

impl<T> Value<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            prev: BTreeSet::new(),
        }
    }
    pub fn new_with_children(data: T, children: BTreeSet<Self>) -> Self {
        Self {
            data,
            prev: children,
        }
    }
}

// Lifetimes for the Value reference inside the BTreeSet are already getting a little gnarley - I will probably want to instead use Rc shared pointers?
// impl<'a, T: ops::Add<Output = T> + cmp::Ord + Copy> ops::Add for &'a Value<T> {
impl<T: ops::Add<Output = T> + cmp::Ord + Copy> ops::Add for Value<T> {
    type Output = Value<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Value::new_with_children(self.data + rhs.data, BTreeSet::from([self, rhs]))
    }
}

impl<T: ops::Mul<Output = T>> ops::Mul for Value<T> {
    type Output = Value<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Value::new(self.data * rhs.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new() {
        let v1 = Value::new(1i32);
        assert_eq!(v1.data, 1i32);
    }

    #[test]
    fn can_add_value() {
        let v1 = Value::new(1i32);
        let v2 = Value::new(2i32);
        let v3 = v1.clone() + v2;
        println!("{}", v3);
        println!("{}", v1); // TODO - Impossible without clone on each op
        assert_eq!(v3.data, 3i32);
    }

    #[test]
    fn can_mul_value() {
        let v1 = Value::new(2i32);
        let v2 = Value::new(3i32);
        let v3 = v1 * v2;
        assert_eq!(v3.data, 6i32);
    }
}
