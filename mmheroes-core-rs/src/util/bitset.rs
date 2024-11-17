use core::marker::PhantomData;

pub(crate) struct BitSet<Storage, Element> {
    bits: Storage,
    phantom: PhantomData<Element>,
}

impl<Storage: Default, Element> BitSet<Storage, Element> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl<T: Default, Element> Default for BitSet<T, Element> {
    fn default() -> Self {
        Self {
            bits: T::default(),
            phantom: PhantomData,
        }
    }
}

macro_rules! specialize {
    ($underlying:ty) => {
        impl<Element: Into<$underlying>>
            crate::util::bitset::BitSet<$underlying, Element>
        {
            pub(crate) fn contains(&self, element: Element) -> bool {
                let i = element.into();
                assert!(i as u32 <= <$underlying>::BITS);
                self.bits & (1 << i) != 0
            }

            pub(crate) fn add(&mut self, element: Element) {
                let i = element.into();
                assert!(i as u32 <= <$underlying>::BITS);
                self.bits |= 1 << i;
            }

            pub(crate) fn count(&self) -> usize {
                self.bits.count_ones() as usize
            }
        }

        impl<Element: Into<$underlying>> FromIterator<Element>
            for crate::util::bitset::BitSet<$underlying, Element>
        {
            fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
                let mut set = Self::new();
                for element in iter {
                    set.add(element);
                }
                set
            }
        }
    };
}

specialize!(u16);
