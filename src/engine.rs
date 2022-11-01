use std::collections::BTreeSet;
use std::{cmp, fmt, ops};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Nop,
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value<T> {
    pub data: T,
    pub label: String,
    op: Op,
    prev: BTreeSet<Value<T>>,
}

impl<T: fmt::Display + fmt::Debug> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} (data: {}, op, {:?}, prev: {:?})",
            self.label, self.data, self.op, self.prev
        )
    }
}

impl<T> Value<T> {
    pub fn new(data: T, label: String) -> Self {
        Self {
            data,
            label,
            op: Op::Nop,
            prev: BTreeSet::new(),
        }
    }
    pub fn new_from_op(data: T, label: String, op: Op, children: BTreeSet<Self>) -> Self {
        Self {
            data,
            label,
            op,
            prev: children,
        }
    }
}

// Lifetimes for the Value reference inside the BTreeSet are already getting a little gnarley - I will probably want to instead use Rc shared pointers?
// impl<'a, T: ops::Add<Output = T> + cmp::Ord + Copy> ops::Add for &'a Value<T> {
impl<T: ops::Add<Output = T> + cmp::Ord + Copy> ops::Add for Value<T> {
    type Output = Value<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Value::new_from_op(
            self.data + rhs.data,
            format!("({} {:?} {})", self.label, Op::Add, rhs.label),
            Op::Add,
            BTreeSet::from([self, rhs]),
        )
    }
}

impl<T: ops::Mul<Output = T> + cmp::Ord + Copy> ops::Mul for Value<T> {
    type Output = Value<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Value::new_from_op(
            self.data * rhs.data,
            format!("({} {:?} {})", self.label, Op::Mul, rhs.label),
            Op::Mul,
            BTreeSet::from([self, rhs]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new() {
        let v1 = Value::new(1i32, String::from("v1"));
        assert_eq!(v1.data, 1i32);
    }

    #[test]
    fn can_add_value() {
        let v1 = Value::new(1i32, String::from("v1"));
        let v2 = Value::new(2i32, String::from("v2"));
        let v3 = v1.clone() + v2;
        println!("{}", v3);
        println!("{}", v1); // TODO - Impossible without clone on each op
        assert_eq!(v3.data, 3i32);
    }

    #[test]
    fn can_mul_value() {
        let v1 = Value::new(2i32, String::from("v1"));
        let v2 = Value::new(3i32, String::from("v2"));
        let v3 = v1 * v2;
        assert_eq!(v3.data, 6i32);
    }

    #[test]
    fn can_backprop() {
        let a = Value::new(2, String::from("a"));
        let b = Value::new(-3, String::from("b"));
        let c = Value::new(10, String::from("c"));
        let mut e = a * b;
        e.label = String::from("e");
        let mut d = c + e;
        d.label = String::from("d");
        let f = Value::new(-2, String::from("f"));
        let mut L = f * d;
        L.label = String::from("L");
        println!("{}", L);
        println!("{}", e);
    }
}
