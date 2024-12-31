use std::iter;
use std::slice;

pub trait Addresses<'a, T: 'a> {
    type Iter: Iterator<Item = (&'a T, &'a usize)>;

    // Method to return the iterator
    fn iter(&'a self) -> Self::Iter;
    // pub fn iter() -> Iterator<Item = (T, usize)>;
}

/// The fastest (probably?) but least memory efficient implementation
/// for storing found addresses. Assuming aligned addresses it should
/// be impossible for it to go over around twice (depending on the
/// type we scan for) the size of the scanned process' memory
pub struct AddressesSimple<T: Copy> {
    values: Vec::<T>,
    addresses: Vec::<usize>,
}

impl<'a, T: Copy + 'a> Addresses<'a, T> for AddressesSimple<T> {
    type Iter = iter::Zip<slice::Iter<'a, T>, slice::Iter<'a, usize>>;

    fn iter(&'a self) -> Self::Iter {
        self.values.iter().zip(self.addresses.iter())
    }
}
