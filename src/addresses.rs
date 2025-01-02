use std::fmt::Debug;
use std::iter;
use std::mem;
use std::{any, str::FromStr};

use crate::context::Context;
use crate::memory_reader::{FromLeBytes, MemoryReader, MemoryReaderSimple};
use crate::process::Process;

pub trait Addresses {
    fn new(process: &Process) -> Self
    where
        Self: Sized;
    fn get_type(&self) -> String;
    fn len(&self) -> usize;
    fn scan(&mut self, ctx: &Context, expr: &ScanExpr);
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
    pub fn eval_expr<F, T, ValIter, AddrIter>(
        &self,
        ctx: &Context,
        f_if_true: &mut F,
        vals: ValIter,
        addrs: AddrIter,
    ) where
        F: FnMut(T, usize),
        T: FromStr + Copy + PartialOrd + PartialEq + Debug + FromLeBytes,
        T::Err: Debug,
        ValIter: Iterator<Item = T>,
        AddrIter: Iterator<Item = usize>,
        [(); mem::size_of::<T>()]:,
    {
        match self {
            Self::Equal(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let mut f_expr = |val, _| val == operand;
                Self::loop_over(f_if_true, &mut f_expr, vals, addrs);
            }
            Self::NotEqual(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let mut f_expr = |val, _| val != operand;
                Self::loop_over(f_if_true, &mut f_expr, vals, addrs);
            }
            Self::Less(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let mut f_expr = |val, _| val < operand;
                Self::loop_over(f_if_true, &mut f_expr, vals, addrs);
            }
            Self::LessEqual(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let mut f_expr = |val, _| val <= operand;
                Self::loop_over(f_if_true, &mut f_expr, vals, addrs);
            }
            Self::Greater(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let mut f_expr = |val, _| val > operand;
                Self::loop_over(f_if_true, &mut f_expr, vals, addrs);
            }
            Self::GreaterEqual(operand) => {
                let operand = operand.parse::<T>().unwrap();
                let mut f_expr = |val, _| val >= operand;
                Self::loop_over(f_if_true, &mut f_expr, vals, addrs);
            }
            Self::Changed => {
                let mut mem_reader = MemoryReaderSimple::new(ctx.process.as_ref().unwrap());
                let mut f_expr = move |val, addr| val != mem_reader.read(addr);
                Self::loop_over(f_if_true, &mut f_expr, vals, addrs);
            }
            // Self::Unknown => {
            //     let f_expr = |lhs, rhs| true;
            //     Self::loop_over(f_if_true, f_expr, vals, addrs, T::defa);
            // }
            _ => panic!("expr doesn't exist"),
        }
    }

    fn loop_over<F, FExpr, T, ValIter, AddrIter>(
        f_if_true: &mut F,
        f_expr: &mut FExpr,
        vals: ValIter,
        addrs: AddrIter,
    ) where
        F: FnMut(T, usize),
        FExpr: FnMut(T, usize) -> bool,
        T: Copy + Debug,
        ValIter: Iterator<Item = T>,
        AddrIter: Iterator<Item = usize>,
    {
        vals.zip(addrs)
            .filter(|(val, addr)| f_expr(*val, *addr))
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
    U: MemoryReader,
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
    [(); mem::size_of::<T>()]:,
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

    fn scan(&mut self, ctx: &Context, expr: &ScanExpr) {
        if self.values.len() != 0 {
            self.noninitial_scan(ctx, expr);
        } else {
            self.initial_scan(ctx, expr);
        }
    }
}

impl<T, U> AddrsSimple<T, U>
where
    T: FromLeBytes + Debug + FromStr + Copy + PartialOrd + PartialEq + 'static,
    T::Err: Debug,
    U: MemoryReader + 'static,
    [(); mem::size_of::<T>()]:,
{
    fn noninitial_scan(&mut self, ctx: &Context, expr: &ScanExpr) {
        let old_vals = mem::take(&mut self.values);
        let old_addrs = mem::take(&mut self.addresses);
        let mut f_if_true = |val: T, addr: usize| {
            self.values.push(val);
            self.addresses.push(addr);
        };
        expr.eval_expr(
            ctx,
            &mut f_if_true,
            old_vals.into_iter(),
            old_addrs.into_iter(),
        );
    }

    fn initial_scan(&mut self, ctx: &Context, expr: &ScanExpr) {
        for memory_map in ctx.process.as_ref().unwrap().memory_maps.iter() {
            if !memory_map.perms.read {
                continue;
            }
            let step = std::mem::size_of::<T>();
            let addrs = (memory_map.addr_start..memory_map.addr_end).step_by(step);
            let mut addrs_cpy = addrs.clone();
            let vals = iter::from_fn(|| match addrs_cpy.next() {
                Some(addr) => Some(self.memory_reader.read::<T>(addr)),
                None => None,
            });
            let mut f_if_true = |val: T, addr: usize| {
                self.values.push(val);
                self.addresses.push(addr);
            };
            expr.eval_expr(ctx, &mut f_if_true, vals, addrs);
        }
    }
}

mod tests {
    use super::*;
    use crate::memory_reader::MemoryReaderSimple;
    use std::process;

    #[test]
    fn scan_addrs_simple() {
        let mut ctx = Context::new();
        ctx.process = Some(Process::try_new(process::id()).unwrap());
        let process = ctx.process.as_ref().unwrap();
        let weird_numbers = vec![0xc0ffee, 0xc0ffee, 0xc0ffee];
        let scan_expr = ScanExpr::Equal(weird_numbers[0].to_string());
        let mut addrs = AddrsSimple::<i32, MemoryReaderSimple>::new(process);
        addrs.scan(&ctx, &scan_expr);

        assert!(addrs.len() >= weird_numbers.len());

        let addr1 = (&weird_numbers[0] as *const i32) as usize;
        let addr2 = (&weird_numbers[1] as *const i32) as usize;
        let addr3 = (&weird_numbers[2] as *const i32) as usize;
        assert!(addrs.get_addrs().contains(&addr1));
        assert!(addrs.get_addrs().contains(&addr2));
        assert!(addrs.get_addrs().contains(&addr3));
    }
}
