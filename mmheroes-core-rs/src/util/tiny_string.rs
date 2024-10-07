use super::*;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::ops::Deref;

#[derive(Clone)]
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
        unsafe { core::str::from_utf8_unchecked(&self.v) }
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
}

impl<const CAPACITY: usize> PartialEq<str> for TinyString<CAPACITY> {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(&self[..], other)
    }
}

impl<'a, const CAPACITY: usize> PartialEq<&'a str> for TinyString<CAPACITY> {
    fn eq(&self, other: &&'a str) -> bool {
        PartialEq::eq(&self[..], &other[..])
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
