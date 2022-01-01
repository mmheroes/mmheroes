#![macro_use]

use core::cmp::Ordering;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::iter::{FromIterator, IntoIterator};
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

pub(crate) struct TinyVec<T, const CAPACITY: usize> {
    storage: [MaybeUninit<T>; CAPACITY],
    count: usize,
}

impl<T, const CAPACITY: usize> TinyVec<T, CAPACITY> {
    pub(crate) fn len(&self) -> usize {
        self.count
    }
}

impl<T, const CAPACITY: usize> TinyVec<T, CAPACITY> {
    pub(crate) fn new() -> Self {
        Self {
            // This should be replaced with [MaybeUninit::uninit(); CAPACITY]
            // as soon as the corresponding feature is stabilized.
            // For now we use this workaround, see // See http://doc.rust-lang.org/1.51.0/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
            storage: unsafe { MaybeUninit::uninit().assume_init() },
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

pub struct TinyString<const CAPACITY: usize> {
    v: TinyVec<u8, CAPACITY>,
}

impl<const CAPACITY: usize> TinyString<CAPACITY> {
    pub(crate) fn new() -> Self {
        Self { v: TinyVec::new() }
    }

    pub(crate) fn push(&mut self, ch: char) {
        match ch.len_utf8() {
            1 => self.v.push(ch as u8),
            _ => self
                .v
                .extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }
}
impl<const CAPACITY: usize> From<&str> for TinyString<CAPACITY> {
    fn from(s: &str) -> Self {
        Self {
            v: s.bytes().collect(),
        }
    }
}

impl<const CAPACITY: usize> Deref for TinyString<CAPACITY> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { core::str::from_utf8_unchecked(&*self.v) }
    }
}

impl<const CAPACITY: usize> FromIterator<char> for TinyString<CAPACITY> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut v = Self::new();
        for element in iter {
            v.push(element);
        }
        v
    }
}

impl<const N: usize, const M: usize> PartialEq<TinyString<M>> for TinyString<N> {
    fn eq(&self, other: &TinyString<M>) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
    fn ne(&self, other: &TinyString<M>) -> bool {
        PartialEq::ne(&self[..], &other[..])
    }
}

impl<const CAPACITY: usize> PartialEq<str> for TinyString<CAPACITY> {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
    fn ne(&self, other: &str) -> bool {
        PartialEq::ne(&self[..], &other[..])
    }
}

impl<'a, const CAPACITY: usize> PartialEq<&'a str> for TinyString<CAPACITY> {
    fn eq(&self, other: &&'a str) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
    fn ne(&self, other: &&'a str) -> bool {
        PartialEq::ne(&self[..], &other[..])
    }
}

impl<const CAPACITY: usize> Debug for TinyString<CAPACITY> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self[..], f)
    }
}

impl<const CAPACITY: usize> core::fmt::Display for TinyString<CAPACITY> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        core::fmt::Display::fmt(&self[..], f)
    }
}

/// В переданной шкале пар `scale` находит первую пару, первый элемент которой строго
/// больше чем `value`, и возвращает второй элемент этой пары. Если такая пара не найдена,
/// возвращает `default`.
pub(crate) fn assess<'a, 'b: 'a, T: PartialOrd, U>(
    scale: &'a [(T, U)],
    value: &T,
    default: &'b U,
) -> &'a U {
    scale
        .iter()
        .find(|&(bound, _)| bound > value)
        .map_or(default, |(_, assessment)| assessment)
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
