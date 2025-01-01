use std::fmt::Debug;
use std::{any, str::FromStr};

use crate::process::Process;

pub trait Addresses {
    fn new() -> Self
    where
        Self: Sized;
    fn get_type(&self) -> String;
    fn len(&self) -> usize;
    fn scan(&self, process: &Process, expr: &ScanExpr) -> Box<dyn Addresses>;
}

#[derive(Debug)]
pub enum ScanExpr {
    Less(String),
    LessEqual(String),
    Greater(String),
    GreaterEqual(String),
    Equal(String),
    NotEqual(String),
    Changed,
    NotChanged,
    Unknown,
}

impl ScanExpr {
    /// Evaluate our expression with every argument from vals. When
    /// the expression is true execute function f_if_true. Typically
    /// we want the function to add filtered values to some other
    /// container
    fn eval_expr<F, T, I1, I2>(&self, f_if_true: &mut F, vals: I1, addrs: I2)
    where
        F: FnMut(T, usize),
        T: FromStr + Copy + PartialOrd + PartialEq,
        T::Err: Debug,
        I1: Iterator<Item = T>,
        I2: Iterator<Item = usize>,
    {
        match self {
            Self::Less(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let f_expr = |lhs, rhs| lhs < rhs;
                Self::loop_over(f_if_true, f_expr, vals, addrs, operand);
            }
            Self::LessEqual(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let f_expr = |lhs, rhs| lhs <= rhs;
                Self::loop_over(f_if_true, f_expr, vals, addrs, operand);
            }
            _ => panic!("expr doesn't exist"),
        }
    }

    fn loop_over<F, FExpr, T, I1, I2>(
        f_if_true: &mut F,
        f_expr: FExpr,
        vals: I1,
        addrs: I2,
        operand: T,
    ) where
        F: FnMut(T, usize),
        FExpr: Fn(T, T) -> bool,
        T: Copy,
        I1: Iterator<Item = T>,
        I2: Iterator<Item = usize>,
    {
        vals.zip(addrs)
            .filter(|(val, _)| f_expr(*val, operand))
            .for_each(|(val, addr)| f_if_true(val, addr));
    }
}

/// The fastest (probably?) but least memory efficient implementation
/// for storing found addresses. Assuming aligned addresses it should
/// be impossible for it to go over around twice (depending on the
/// type we scan for) the size of the scanned process' memory
pub struct AddrsSimple<T: Copy> {
    values: Vec<T>,
    addresses: Vec<usize>,
}

impl<T: Copy + 'static> Addresses for AddrsSimple<T> {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            addresses: Vec::new(),
        }
    }

    fn get_type(&self) -> String {
        any::type_name::<T>().to_string()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn scan(&self, process: &Process, expr: &ScanExpr) -> Box<dyn Addresses> {
        let mut ret = Box::new(AddrsSimple::<T>::new());

        ret
    }
}
