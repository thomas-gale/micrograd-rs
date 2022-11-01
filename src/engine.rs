use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
use std::{cmp, fmt, ops};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Nop,
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueData<T> {
    pub data: T,
    pub label: String,
    op: Op,
    prev: BTreeSet<Value<T>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value<T> {
    val: Rc<RefCell<ValueData<T>>>,
}

impl<T: fmt::Display + fmt::Debug> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} (data: {}, op, {:?}, prev: {:?})",
            self.val.as_ref().borrow().label,
            self.val.as_ref().borrow().data,
            self.val.as_ref().borrow().op,
            self.val.as_ref().borrow().prev
        )
    }
}

impl<T: Clone> Value<T> {
    pub fn new(data: T, label: String) -> Self {
        Self {
            val: Rc::new(RefCell::new(ValueData {
                data,
                label,
                op: Op::Nop,
                prev: BTreeSet::new(),
            })),
        }
    }

    pub fn new_from_op(data: T, label: String, op: Op, children: BTreeSet<Self>) -> Self {
        Self {
            val: Rc::new(RefCell::new(ValueData {
                data,
                label,
                op,
                prev: children,
            })),
        }
    }

    pub fn data(&self) -> T {
        self.val.as_ref().borrow().data.clone()
    }

    pub fn label(&self) -> String {
        self.val.as_ref().borrow().label.clone()
    }

    pub fn set_label(&mut self, label: String) {
        self.val.as_ref().borrow_mut().label = label;
    }
}

// Lifetimes for the Value reference inside the BTreeSet are already getting a little gnarley - I will probably want to instead use Rc shared pointers?
// impl<'a, T: ops::Add<Output = T> + cmp::Ord + Copy> ops::Add for &'a Value<T> {
impl<T: ops::Add<Output = T> + cmp::Ord + Copy> ops::Add for Value<T> {
    type Output = Value<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Value::new_from_op(
            self.val.as_ref().borrow().data + rhs.val.as_ref().borrow().data,
            format!(
                "({} {:?} {})",
                self.val.as_ref().borrow().label,
                Op::Add,
                rhs.val.as_ref().borrow().label
            ),
            Op::Add,
            BTreeSet::from([self.clone(), rhs.clone()]),
        )
    }
}

impl<T: ops::Mul<Output = T> + cmp::Ord + Copy> ops::Mul for Value<T> {
    type Output = Value<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Value::new_from_op(
            self.val.as_ref().borrow().data * rhs.val.as_ref().borrow().data,
            format!(
                "({} {:?} {})",
                self.val.as_ref().borrow().label,
                Op::Mul,
                rhs.val.as_ref().borrow().label
            ),
            Op::Mul,
            BTreeSet::from([self.clone(), rhs.clone()]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new() {
        let v1 = Value::new(1i32, String::from("v1"));
        assert_eq!(v1.val.as_ref().borrow().data, 1i32);
    }

    #[test]
    fn can_add_value() {
        let v1 = Value::new(1i32, String::from("v1"));
        let v2 = Value::new(2i32, String::from("v2"));
        let v3 = v1.clone() + v2;
        println!("{}", v3);
        println!("{}", v1); // TODO - Impossible without clone on each op
        assert_eq!(v3.val.as_ref().borrow().data, 3i32);
    }

    #[test]
    fn can_mul_value() {
        let v1 = Value::new(2i32, String::from("v1"));
        let v2 = Value::new(3i32, String::from("v2"));
        let v3 = v1 * v2;
        assert_eq!(v3.val.as_ref().borrow().data, 6i32);
    }

    #[test]
    fn can_backprop() {
        let a = Value::new(2, String::from("a"));
        let b = Value::new(-3, String::from("b"));
        let c = Value::new(10, String::from("c"));
        let mut e = a * b;
        e.set_label(String::from("e"));
        let mut d = c + e;
        d.set_label(String::from("d"));
        let f = Value::new(-2, String::from("f"));
        let mut L = f * d;
        L.set_label(String::from("d"));
        println!("{}", L);
        // println!("{}", e);
    }
}
