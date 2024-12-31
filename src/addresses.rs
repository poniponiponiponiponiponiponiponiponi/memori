use std::iter;
use std::slice;
use std::any;

pub trait Addresses {
    fn new() -> Self where Self: Sized;
    fn get_type(&self) -> String;
}

/// The fastest (probably?) but least memory efficient implementation
/// for storing found addresses. Assuming aligned addresses it should
/// be impossible for it to go over around twice (depending on the
/// type we scan for) the size of the scanned process' memory
pub struct AddrsSimple<T: Copy> {
    values: Vec<T>,
    addresses: Vec<usize>,
}

impl<T: Copy> Addresses for AddrsSimple<T> {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            addresses: Vec::new(),
        }
    }

    fn get_type(&self) -> String {
        any::type_name::<T>().to_string()
    }
}


