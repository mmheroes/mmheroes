#![macro_use]
#![allow(clippy::redundant_slicing)]

use core::cmp::Ordering;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::iter::IntoIterator;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

pub(crate) struct TinyVec<T, const CAPACITY: usize> {
    storage: [MaybeUninit<T>; CAPACITY],
    count: usize,
}

impl<T, const CAPACITY: usize> TinyVec<T, CAPACITY> {
    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        self.count
    }
}

impl<T, const CAPACITY: usize> TinyVec<T, CAPACITY> {
    pub(crate) fn new() -> Self {
        Self {
            storage: [const { MaybeUninit::uninit() }; CAPACITY],
            count: 0,
        }
    }

    pub(crate) fn push(&mut self, value: T) {
        assert!(self.count < self.storage.len(), "Capacity is exceeded");
        unsafe { self.storage[self.count].as_mut_ptr().write(value) }
        self.count += 1;
    }

    pub(crate) fn clear(&mut self) {
        for element in self.storage.iter_mut().take(self.count) {
            unsafe { core::ptr::drop_in_place(element.as_mut_ptr()) }
        }
        self.count = 0
    }
}

impl<T, const CAPACITY: usize> Deref for TinyVec<T, CAPACITY> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let slice = &self.storage[..self.count];
        unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
    }
}

impl<T, const CAPACITY: usize> DerefMut for TinyVec<T, CAPACITY> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let slice = &mut self.storage[..self.count];
        unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }
    }
}

impl<T, Index: core::slice::SliceIndex<[T]>, const CAPACITY: usize>
    core::ops::Index<Index> for TinyVec<T, CAPACITY>
{
    type Output = <Index as core::slice::SliceIndex<[T]>>::Output;

    fn index(&self, index: Index) -> &Self::Output {
        core::ops::Index::index(&**self, index)
    }
}

impl<T, const N: usize, const CAPACITY: usize> From<[T; N]> for TinyVec<T, CAPACITY> {
    fn from(arr: [T; N]) -> Self {
        assert!(N <= CAPACITY); // Must be a static assert
        let mut vec = Self::new();
        for elem in arr {
            vec.push(elem)
        }
        vec
    }
}

impl<T: Clone, const CAPACITY: usize> TinyVec<T, CAPACITY> {
    pub(crate) fn extend_from_slice(&mut self, other: &[T]) {
        for element in other {
            self.push(element.clone())
        }
    }
}

impl<T: Clone, const CAPACITY: usize> Clone for TinyVec<T, CAPACITY> {
    fn clone(&self) -> Self {
        let mut result = Self::new();
        for element in &**self {
            result.push(element.clone());
        }
        result
    }
}

impl<T: Debug, const CAPACITY: usize> Debug for TinyVec<T, CAPACITY> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&**self, f)
    }
}

impl<T: PartialEq, const CAPACITY: usize> PartialEq for TinyVec<T, CAPACITY> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<T: Eq, const CAPACITY: usize> Eq for TinyVec<T, CAPACITY> {}

impl<T: PartialOrd, const CAPACITY: usize> PartialOrd for TinyVec<T, CAPACITY> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<T: Ord, const CAPACITY: usize> Ord for TinyVec<T, CAPACITY> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<T, const CAPACITY: usize> FromIterator<T> for TinyVec<T, CAPACITY> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = Self::new();
        for element in iter {
            v.push(element);
        }
        v
    }
}

impl<T, const CAPACITY: usize> Extend<T> for TinyVec<T, CAPACITY> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for element in iter {
            self.push(element);
        }
    }
}

impl<T, const CAPACITY: usize> Drop for TinyVec<T, CAPACITY> {
    fn drop(&mut self) {
        self.clear()
    }
}

macro_rules! __tiny_vec_push_elements {
    ($v:ident, ) => ();
    ($v:ident, $head:expr, $($tail:expr,)*) => {
        $v.push($head);
        __tiny_vec_push_elements!($v, $($tail,)*);
    };
}

#[allow(unused_macros)]
macro_rules! tiny_vec {
    (capacity: $capacity:literal) => (
        <crate::util::TinyVec<_, $capacity>>::new()
    );
    (capacity: $capacity:literal, []) => (
        tiny_vec!(capacity: $capacity)
    );
    (capacity: $capacity:literal, [$($elements:expr),*]) => {{
        let mut v = tiny_vec!(capacity: $capacity);
        __tiny_vec_push_elements!(v, $($elements,)*);
        v
    }};
    (capacity: $capacity:literal, [$($elements:expr,)*]) => (
        tiny_vec![capacity: $capacity, [$($elements),*]]
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_full_capacity() {
        let mut v = tiny_vec!(capacity: 16);
        for i in 0..16 {
            v.push(i + 1);
        }
        assert_eq!(&*v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }

    #[test]
    #[should_panic]
    fn test_exceeding_capacity() {
        let mut v = tiny_vec!(capacity: 16);
        for i in 0..=16 {
            v.push(i + 1);
        }
    }

    #[test]
    fn test_clone() {
        let v = tiny_vec!(capacity: 16, [1, 3, 5, 7, 9]);
        let cloned = v.clone();
        assert_eq!(&*cloned, [1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_clear() {
        use core::cell::RefCell;

        let drop_counter = RefCell::new(0usize);

        struct DropCounter<'a> {
            counter: &'a RefCell<usize>,
        }

        impl Drop for DropCounter<'_> {
            fn drop(&mut self) {
                *self.counter.borrow_mut() += 1;
            }
        }

        let vec = tiny_vec!(capacity: 16, [
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
            DropCounter { counter: &drop_counter },
        ]);

        assert_eq!(*drop_counter.borrow(), 0);

        core::mem::drop(vec);

        assert_eq!(*drop_counter.borrow(), 5);
    }
}
