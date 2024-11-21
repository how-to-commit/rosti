#![cfg_attr(not(test), no_std)]

// use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

#[repr(transparent)]
pub struct KVolatile<T: Copy>(T);

impl<T: Copy> KVolatile<T> {
    pub const fn new(value: T) -> KVolatile<T> {
        KVolatile(value)
    }

    pub fn read(&self) -> T {
        unsafe { read_volatile(&self.0) }
    }

    pub fn write(&mut self, value: T) {
        unsafe { write_volatile(&mut self.0, value) }
    }
}
// #[repr(transparent)]
// pub struct KVolatile<'a, T>
// where
//     T: ?Sized + Copy,
// {
//     pointer: NonNull<T>,
//     _marker: PhantomData<&'a T>,
// }

// impl<'a, T> KVolatile<'a, T>
// where
//     T: ?Sized + Copy,
// {
//     pub const fn new(pointer: NonNull<T>) -> KVolatile<'a, T> {
//         let _marker = PhantomData;
//         KVolatile { pointer, _marker }
//     }

//     pub fn read(&self) -> T {
//         unsafe { read_volatile(self.pointer.as_ptr()) }
//     }

//     pub fn write(&self, value: T) {
//         unsafe { write_volatile(self.pointer.as_ptr(), value) }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn basic_test() {
    //     let value = 42;
    //     let wrapped_value = KVolatile::new(NonNull::from(&value));
    //     assert_eq!(wrapped_value.read(), 42);

    //     wrapped_value.write(54);
    //     assert_eq!(wrapped_value.read(), 54);
    // }

    #[test]
    fn basic_test() {
        let value = 42;
        let mut wrapped_value = KVolatile::new(value);
        assert_eq!(wrapped_value.read(), 42);

        wrapped_value.write(54);
        assert_eq!(wrapped_value.read(), 54);
    }
}
