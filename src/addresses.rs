use std::iter;
use std::slice;
use std::any;

pub trait Addresses<'a, T: 'a> {
    type Iter: Iterator<Item = (&'a T, &'a usize)>;

    fn new() -> Self;
    fn iter(&'a self) -> Self::Iter;
    fn push(&mut self, value: T, addr: usize);

    fn get_type(&self) -> String {
        any::type_name::<T>().to_string()
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

impl<'a, T: Copy + 'a> Addresses<'a, T> for AddrsSimple<T> {
    type Iter = iter::Zip<slice::Iter<'a, T>, slice::Iter<'a, usize>>;

    fn new() -> Self {
        Self {
            values: Vec::new(),
            addresses: Vec::new(),
        }
    }

    fn iter(&'a self) -> Self::Iter {
        self.values.iter().zip(self.addresses.iter())
    }

    fn push(&mut self, value: T, addr: usize) {
        self.values.push(value);
        self.addresses.push(addr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_simple_iter_test() {
        let addrs = AddrsSimple {
            values: vec![1, 2, 3],
            addresses: vec![0x11, 0x22, 0x33],
        };

        let mut addrs_iter = addrs.iter();
        assert_eq!(addrs_iter.next(), Some((&1, &0x11usize)));
        assert_eq!(addrs_iter.next(), Some((&2, &0x22usize)));
        assert_eq!(addrs_iter.next(), Some((&3, &0x33usize)));
        assert_eq!(addrs_iter.next(), None);
    }
}
