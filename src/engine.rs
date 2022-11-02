use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;
use std::{fmt, ops};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Op {
    Nop,
    Add,
    Mul,
    Tanh,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ValueData<T> {
    pub data: T,
    pub label: String,
    op: Op,
    prev: Vec<Value<T>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Value<T> {
    val: Rc<RefCell<ValueData<T>>>,
}

impl<T: fmt::Display + fmt::Debug + Clone> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} (data: {}, op, {:?}, prev: {:?})",
            self.val.as_ref().borrow().label,
            self.val.as_ref().borrow().data,
            self.val.as_ref().borrow().op,
            self.val
                .as_ref()
                .borrow()
                .prev
                .iter()
                .map(|v| v.label())
                .collect::<Vec<String>>()
        )
    }
}

// Utility function implementations
impl<T: Clone> Value<T> {
    pub fn new(data: T, label: String) -> Self {
        Self {
            val: Rc::new(RefCell::new(ValueData {
                data,
                label,
                op: Op::Nop,
                prev: vec![],
            })),
        }
    }
    pub fn new_from_op(data: T, label: String, op: Op, children: Vec<Self>) -> Self {
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

impl<'a, T: ops::Add<Output = T> + Clone> ops::Add for &'a Value<T> {
    type Output = Value<T>;
    fn add(self, rhs: Self) -> Value<T> {
        let op = Op::Add;
        let t = self.data() + rhs.data();
        Value::new_from_op(
            t,
            format!("({} {:?} {})", self.label(), op, rhs.label()),
            op,
            vec![self.clone(), rhs.clone()],
        )
    }
}

impl<'a, T: ops::Mul<Output = T> + Clone> ops::Mul for &'a Value<T> {
    type Output = Value<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let op = Op::Mul;
        Value::new_from_op(
            self.data() * rhs.data(),
            format!("({} {:?} {})", self.label(), op, rhs.label()),
            op,
            vec![self.clone(), rhs.clone()],
        )
    }
}

pub trait Exp {
    fn exp(self) -> Self;
}

impl Exp for f64 {
    fn exp(self) -> Self {
        self.exp()
    }
}

impl<'a, T> Value<T>
where
    T: Mul<f64, Output = T>
        + Div<T, Output = T>
        + Add<f64, Output = T>
        + Sub<f64, Output = T>
        + Exp
        + Clone,
{
    pub fn tanh(&self) -> Self {
        let op = Op::Tanh;
        let e = (self.data() * 2.0).exp();
        let t = (e.clone() - 1.0) / (e.clone() + 1.0);
        Value::new_from_op(t, format!("tanh({})", self.label()), op, vec![self.clone()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new() {
        let v1i = Value::new(1, String::from("v1"));
        assert_eq!(v1i.data(), 1);

        let v1f = Value::new(1.0, String::from("v1"));
        assert_eq!(v1f.data(), 1.0);
    }

    #[test]
    fn can_add_value() {
        let v1i = Value::new(1, String::from("v1"));
        let v2i = Value::new(2, String::from("v2"));
        let v3i = &v1i + &v2i;
        assert_eq!(v3i.data(), 3);

        let v1f = Value::new(1.0, String::from("v1"));
        let v2f = Value::new(2.0, String::from("v2"));
        let v3f = &v1f + &v2f;
        assert_eq!(v3f.data(), 3.0);
    }

    #[test]
    fn can_mul_value() {
        let v1i = Value::new(2, String::from("v1"));
        let v2i = Value::new(3, String::from("v2"));
        let v3i = &v1i * &v2i;
        assert_eq!(v3i.data(), 6);

        let v1f = Value::new(2.0, String::from("v1"));
        let v2f = Value::new(3.0, String::from("v2"));
        let v3f = &v1f * &v2f;
        assert_eq!(v3f.data(), 6.0);
    }

    #[test]
    fn sample_graph() {
        let a = Value::new(2, String::from("a"));
        let b = Value::new(-3, String::from("b"));
        let c = Value::new(10, String::from("c"));
        let mut e = &a * &b;
        e.set_label(String::from("e"));
        let mut d = &c + &e;
        d.set_label(String::from("d"));
        let f = Value::new(-2, String::from("f"));
        let mut l = &f * &d;
        l.set_label(String::from("d"));
        println!("{}", l);
        println!("{}", e);
        println!("{}", b);
    }

    #[test]
    fn sample_neuron() {
        let x1 = Value::new(2.0, String::from("x1"));
        let x2 = Value::new(0.0, String::from("x2"));

        let w1 = Value::new(-3.0, String::from("w1"));
        let w2 = Value::new(1.0, String::from("w2"));

        let b = Value::new(6.88137, String::from("b"));

        let x1w1 = &x1 * &w1;
        let x2w2 = &x2 * &w2;
        let x1w1x2w2 = &x1w1 + &x2w2;

        let mut n = &x1w1x2w2 + &b;
        n.set_label(String::from("n"));
        let mut o = n.tanh();
        o.set_label(String::from("o"));
        println!("{}", o);
        println!("{}", n);
    }
}
