use std::iter;
use std::slice;

pub trait Addresses<'a, T: 'a> {
    type Iter: Iterator<Item = (&'a T, &'a usize)>;

    fn iter(&'a self) -> Self::Iter;
}

/// The fastest (probably?) but least memory efficient implementation
/// for storing found addresses. Assuming aligned addresses it should
/// be impossible for it to go over around twice (depending on the
/// type we scan for) the size of the scanned process' memory
pub struct AddressesSimple<T: Copy> {
    values: Vec<T>,
    addresses: Vec<usize>,
}

impl<'a, T: Copy + 'a> Addresses<'a, T> for AddressesSimple<T> {
    type Iter = iter::Zip<slice::Iter<'a, T>, slice::Iter<'a, usize>>;

    fn iter(&'a self) -> Self::Iter {
        self.values.iter().zip(self.addresses.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_simple_iter_test() {
        let addrs = AddressesSimple {
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
