use std::fmt::Debug;
use std::{any, str::FromStr};
use std::mem;
use std::iter;

use crate::memory_reader::{FromLeBytes, MemoryReader};
use crate::process::Process;

pub trait Addresses {
    fn new(process: &Process) -> Self
    where
        Self: Sized;
    fn get_type(&self) -> String;
    fn len(&self) -> usize;
    fn scan(&mut self, process: &Process, expr: &ScanExpr) -> Box<dyn Addresses>;
    fn get_addrs(&self) -> Vec<usize>;
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
    pub fn eval_expr<F, T, ValIter, AddrIter>(&self, f_if_true: &mut F, vals: ValIter, addrs: AddrIter)
    where
        F: FnMut(T, usize),
        T: FromStr + Copy + PartialOrd + PartialEq + Debug,
        T::Err: Debug,
        ValIter: Iterator<Item = T>,
        AddrIter: Iterator<Item = usize>,
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
            Self::Equal(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let f_expr = |lhs, rhs| lhs == rhs;
                Self::loop_over(f_if_true, f_expr, vals, addrs, operand);
            }
            _ => panic!("expr doesn't exist"),
        }
    }

    fn loop_over<F, FExpr, T, ValIter, AddrIter>(
        f_if_true: &mut F,
        f_expr: FExpr,
        vals: ValIter,
        addrs: AddrIter,
        operand: T,
    ) where
        F: FnMut(T, usize),
        FExpr: Fn(T, T) -> bool,
        T: Copy + Debug,
        ValIter: Iterator<Item = T>,
        AddrIter: Iterator<Item = usize>,
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
#[derive(Debug)]
pub struct AddrsSimple<T, U>
where
    T: FromStr + Copy + PartialOrd + PartialEq,
    U: MemoryReader
{
    values: Vec<T>,
    addresses: Vec<usize>,
    memory_reader: U,
}

impl<T, U> Addresses for AddrsSimple<T, U>
where
    T: FromLeBytes + Debug + FromStr + Copy + PartialOrd + PartialEq + 'static,
    T::Err: Debug,
    U: MemoryReader + 'static,
[(); mem::size_of::<T>()]:
{
    fn new(process: &Process) -> Self {
        Self {
            values: Vec::new(),
            addresses: Vec::new(),
            memory_reader: U::new(&process),
        }
    }

    fn get_addrs(&self) -> Vec<usize> {
        self.addresses.clone()
    }

    fn get_type(&self) -> String {
        any::type_name::<T>().to_string()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn scan(&mut self, process: &Process, expr: &ScanExpr) -> Box<dyn Addresses> {
        if self.values.len() != 0 {
            self.noninitial_scan(process, expr)
        } else {
            self.initial_scan(process, expr)
        }
    }

}

impl<T, U> AddrsSimple<T, U>
where
    T: FromLeBytes + Debug + FromStr + Copy + PartialOrd + PartialEq + 'static,
    T::Err: Debug,
    U: MemoryReader + 'static,
[(); mem::size_of::<T>()]:
{
    fn noninitial_scan(&self, process: &Process, expr: &ScanExpr) -> Box<dyn Addresses> {
        let mut ret = Box::new(AddrsSimple::<T, U>::new(&process));
        let mut new_vals = Vec::<T>::new();
        let mut new_addrs = Vec::<usize>::new();
        let mut f_if_true = |val: T, addr: usize| {
            new_vals.push(val);
            new_addrs.push(addr);
        };
        // TODO remove clone
        expr.eval_expr(&mut f_if_true, self.values.clone().into_iter(), self.addresses.clone().into_iter());
        ret.values = new_vals;
        ret.addresses = new_addrs;
        ret
    }

    fn initial_scan(&mut self, process: &Process, expr: &ScanExpr) -> Box<dyn Addresses> {
        let mut ret = Box::new(AddrsSimple::<T, U>::new(&process));
        for memory_map in process.memory_maps.iter() {
            if !memory_map.perms.read {
                continue;
            }
            let step = std::mem::size_of::<T>();
            let addrs = (memory_map.addr_start..memory_map.addr_end).step_by(step);
            let mut addrs_cpy = addrs.clone();
            let vals = iter::from_fn(|| {
                match addrs_cpy.next() {
                    Some(addr) => {
                        Some(self.memory_reader.read::<T>(addr))
                    },
                    None => {
                        None
                    }
                }
            });
            let mut new_vals = Vec::new();
            let mut new_addrs = Vec::new();
            let mut f_if_true = |val: T, addr: usize| {
                new_vals.push(val);
                new_addrs.push(addr);
            };
            expr.eval_expr(&mut f_if_true, vals, addrs);
            ret.values.append(&mut new_vals);
            ret.addresses.append(&mut new_addrs);
        }
        ret
    }
}

mod tests {
    use crate::memory_reader::MemoryReaderSimple;
    use super::*;
    use std::process;

    #[test]
    fn scan_addrs_simple() {
        let self_proc = Process::try_new(process::id()).unwrap();
        let weird_numbers = vec![0xc0ffee, 0xc0ffee, 0xc0ffee];
        let scan_expr = ScanExpr::Equal(weird_numbers[0].to_string());
        let mut addrs = AddrsSimple::<i32, MemoryReaderSimple>::new(&self_proc);
        let after_scan = addrs.scan(&self_proc, &scan_expr);
        
        assert!(after_scan.len() >= weird_numbers.len());

        let addr1 = (&weird_numbers[0] as *const i32) as usize;
        let addr2 = (&weird_numbers[1] as *const i32) as usize;
        let addr3 = (&weird_numbers[2] as *const i32) as usize;
        assert!(after_scan.get_addrs().contains(&addr1));
        assert!(after_scan.get_addrs().contains(&addr2));
        assert!(after_scan.get_addrs().contains(&addr3));
    }
}
