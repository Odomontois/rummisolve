use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    mem::take,
    ops::{Add, AddAssign, IndexMut, SubAssign},
    slice::SliceIndex,
};

use derivative::Derivative;
use std::fmt::Debug;
use std::num::NonZero;
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
struct ElementHeader<C, I> {
    first: Option<I>,
    amount: C,
}

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(bound = ""))]
struct SetHeader<Idx> {
    elements: Vec<Idx>,
    deleted: bool,
}

impl<C: Addressable, I> Default for ElementHeader<C, I> {
    fn default() -> Self {
        Self {
            first: None,
            amount: C::ZERO,
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Backstack<I> {
    Cell(I),
    Set(I),
    Element(I),
    Chosen { cell: I },
}

const DONE: Option<()> = Some(());
const FAIL: Option<()> = None;

#[derive(Clone, Derivative, Debug)]
#[derivative(Default(bound = ""))]
struct DancingLinks<C, I> {
    sets: Vec<SetHeader<I>>,
    elements: Vec<ElementHeader<C, I>>,
    cells: Vec<Cell<I>>,
    backstack: Vec<Backstack<I>>,
    element_choose: BTreeSet<(C, I)>,
}

type DancingLinksOf<C> = DancingLinks<C, <C as Addressable>::Idx>;

impl<C: Addressable<Idx = I>, I: AddressableIdx> DancingLinks<C, I> {
    fn new(xs: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let mut dl = Self::default();
        let mut builder = builder::DancingLinksBuilder::new(&mut dl);

        for (set, elem) in xs {
            builder.add_link(I::from_address(set), I::from_address(elem));
        }

        dl
    }

    fn on_element_change(&mut self, i: I, f: impl FnOnce(&mut C)) {
        let hd = &mut self.elements[Ix(i)];
        self.element_choose.remove(&(hd.amount, i));
        f(&mut hd.amount);
        self.element_choose.insert((hd.amount, i));
    }

    fn set_elements(&self, ix: I) -> &[I] {
        &self.sets[Ix(ix)].elements
    }

    fn remove_set(&mut self, set_ix: I) -> Option<()> {
        for elem_pos in 0..self.set_elements(set_ix).len() {
            let elem_ix = self.set_elements(set_ix)[elem_pos];
            self.remove_cell(elem_ix)?;
        }
        DONE
    }

    fn remove_cell(&mut self, i: I) -> Option<()> {
        let &cur = &self.cells[Ix(i)];
        let hi = cur.element.address();
        let hd = &mut self.elements[hi];
        debug_assert!(hd.first == Some(i));
        if let Some(prev_set) = cur.prev_set {
            self.cells[Ix(prev_set)].next_set = cur.next_set;
        } else {
            hd.first = cur.next_set;
        }
        if let Some(next_set) = cur.next_set {
            self.cells[Ix(next_set)].prev_set = cur.prev_set
        }
        if hd.first == None {
            return FAIL;
        }
        self.on_element_change(i, C::decrease);
        self.backstack.push(Backstack::Cell(i));
        DONE
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Cell<I> {
    prev_set: Option<I>,
    next_set: Option<I>,
    set: I,
    element: I,
}

trait Addressable: Copy + Eq + Ord + SubAssign + AddAssign + 'static {
    type Idx: AddressableIdx;
    const ONE: Self;
    const ZERO: Self;
    fn decrease(&mut self) {
        *self -= Self::ONE
    }

    fn increase(&mut self) {
        *self += Self::ONE
    }
}

trait AddressableIdx:
    Copy + Eq + Ord + TryFrom<NonZero<usize>> + TryInto<NonZero<usize>> + 'static
{
    type Count: Addressable;
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

struct Ix<I>(I);

impl<A, I: AddressableIdx> Index<Ix<I>> for Vec<A> {
    type Output = A;

    fn index(&self, index: Ix<I>) -> &Self::Output {
        &self[index.0.address()]
    }
}

impl<A, I: AddressableIdx> IndexMut<Ix<I>> for Vec<A> {
    fn index_mut(&mut self, index: Ix<I>) -> &mut Self::Output {
        &mut self[index.0.address()]
    }
}
macro_rules! impl_addressable {
    ($($t:ty),*) => {
        $(
            impl AddressableIdx for NonZero<$t> {
                type Count = $t;
                const ONE: Self = NonZero::new(1).unwrap();
            }

            impl Addressable for $t{
                type Idx = NonZero<$t>;
                const ONE: $t = 1;
                const ZERO: $t = 0;
            }
        )*
    };
}

impl_addressable!(u8, u16, u32, u64, usize);

mod builder {
    use std::{cell, collections::HashMap, hash::Hash};

    use super::*;

    #[derive(Derivative, Debug)]
    pub(super) struct DancingLinksBuilder<'a, C: Addressable<Idx = I>, I: AddressableIdx> {
        dl: &'a mut DancingLinks<C, C::Idx>,
    }

    impl<'a, C: Addressable<Idx = I>, I: AddressableIdx> DancingLinksBuilder<'a, C, I> {
        pub(super) fn new(dl: &'a mut DancingLinks<C, C::Idx>) -> Self {
            Self { dl }
        }

        fn grow<X: Default>(data: &mut Vec<X>, ix: I) -> &mut X {
            let ix = ix.address();
            data.resize_with(data.len().max(ix), <_>::default);
            &mut data[ix]
        }

        fn insert_element(&mut self, hidx: I, cell_idx: I) -> Option<I> {
            let elem = Self::grow(&mut self.dl.elements, hidx);
            let old = elem.first;
            elem.first = Some(cell_idx);
            old
        }

        fn insert_set(&mut self, hidx: I, cell_idx: I) {
            Self::grow(&mut self.dl.sets, hidx).elements.push(cell_idx);
        }

        pub(super) fn add_link(&mut self, set: I, element: I) {
            let i = I::from_address(self.dl.cells.len());
            let next_set = self.insert_set(set, i);
            let next_set = self.insert_element(element, i);

            self.dl.cells.push(Cell {
                prev_set: None,
                next_set,
                set,
                element,
            });
        }
    }
}

mod tests {
    use crate::model::solver::{DancingLinks, DancingLinksOf};

    #[test]
    fn check() {
        let u: DancingLinksOf<u8> = DancingLinks::new([]);
    }
}
