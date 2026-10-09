use std::{num::NonZero, ops::{Add, AddAssign, Index, IndexMut, SubAssign}};

pub(crate)  trait Count: Copy + Eq + Ord + SubAssign + AddAssign + 'static {
    type Idx: Address<Count = Self>;
    const ONE: Self;
    const ZERO: Self;
    fn decrease(&mut self) {
        *self -= Self::ONE
    }

    fn increase(&mut self) {
        *self += Self::ONE
    }
}

pub(crate) trait Address: Copy + Eq + Ord + TryFrom<NonZero<usize>> + TryInto<NonZero<usize>> + 'static {
    type Count: Count<Idx = Self>;
    const ONE: Self;
    fn address(self) -> usize {
        self.try_into().map_or(1, <_>::into) - 1
    }

    fn from_address(address: usize) -> Self {
        NonZero::new(address + 1)
            .and_then(|x| x.try_into().ok())
            .unwrap()
    }
}

macro_rules! impl_addressable {
    ($($t:ty),*) => {
        $(
            impl Address for NonZero<$t> {
                type Count = $t;
                const ONE: Self = NonZero::new(1).unwrap();
            }

            impl Count for $t{
                type Idx = NonZero<$t>;
                const ONE: $t = 1;
                const ZERO: $t = 0;
            }
        )*
    };
}

impl_addressable!(u8, u16, u32, u64, usize);

pub(crate) struct Ix<I>(pub(crate) I);

impl<A, I: Address> Index<Ix<I>> for Vec<A> {
    type Output = A;

    fn index(&self, index: Ix<I>) -> &Self::Output {
        &self[index.0.address()]
    }
}

impl<A, I: Address> IndexMut<Ix<I>> for Vec<A> {
    fn index_mut(&mut self, index: Ix<I>) -> &mut Self::Output {
        &mut self[index.0.address()]
    }
}
