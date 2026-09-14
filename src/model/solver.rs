use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    ops::{Add, AddAssign, SubAssign},
    slice::SliceIndex,
};

use derivative::Derivative;
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
struct ElementHeader<I> {
    first: I,
    amount: I,
}

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(bound = ""))]
struct SetHeader<I> {
    elements: Vec<I>,
    deleted: bool,
}

impl<I: Addressable> Default for ElementHeader<I> {
    fn default() -> Self {
        Self {
            first: I::NULL,
            amount: I::ZERO,
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

#[derive(Debug, Clone, Derivative)]
#[derivative(Default(bound = ""))]
struct DancingLinks<I: Addressable> {
    sets: Vec<SetHeader<I>>,
    elements: Vec<ElementHeader<I>>,
    cells: Vec<Cell<I>>,
    backstack: Vec<Backstack<I>>,
    element_choose: BTreeSet<(I, I)>,
}

impl<I: Addressable> DancingLinks<I> {
    fn new(xs: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let mut dl = Self::default();
        let mut builder = builder::DancingLinksBuilder::new(&mut dl);

        for (set, elem) in xs {
            builder.add_link(I::from_address(set), I::from_address(elem));
        }

        dl
    }
    fn cell(&self, i: I) -> Option<&Cell<I>> {
        i.address().map(|i| &self.cells[i])
    }

    fn cell_mut(&mut self, i: I) -> Option<&mut Cell<I>> {
        i.address().map(move |i| &mut self.cells[i])
    }

    fn on_element_change(&mut self, i: I, f: impl FnOnce(&mut I)) {
        let hd = i.get_mut_from(&mut self.elements).unwrap();
        self.element_choose.remove(&(hd.amount, i));
        f(&mut hd.amount);
        self.element_choose.insert((hd.amount, i));
    }

    fn remove_set(&mut self, set: I) -> Option<()> {
        DONE
    }

    fn remove_cell(&mut self, i: I) -> Option<()> {
        let &cur = self.cell(i).unwrap();
        let hi = cur.element.address().unwrap();
        let hd = &mut self.elements[hi];
        debug_assert!(hd.first == i);
        if let Some(prev) = cur.prev_set.get_mut_from(&mut self.cells) {
            prev.next_set = cur.next_set;
        } else {
            hd.first = cur.next_set;
        }
        if let Some(next) = cur.next_set.get_mut_from(&mut self.cells) {
            next.prev_set = cur.prev_set
        }
        if hd.first == I::NULL {
            return FAIL;
        }
        self.on_element_change(i, I::decrease);
        self.backstack.push(Backstack::Cell(i));
        DONE
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Cell<I: Addressable> {
    prev_set: I,
    next_set: I,
    set: I,
    element: I,
}

trait Addressable:
    Copy + Eq + TryInto<usize> + Ord + SubAssign + AddAssign + TryFrom<usize> + 'static
{
    const NULL: Self;
    const ONE: Self;
    const ZERO: Self;
    fn address(self) -> Option<usize> {
        if self == Self::NULL {
            None
        } else {
            self.try_into().ok()
        }
    }

    fn get_from<A>(self, c: &[A]) -> Option<&A> {
        c.get(self.address()?)
    }

    fn get_mut_from<A>(self, c: &mut [A]) -> Option<&mut A> {
        c.get_mut(self.address()?)
    }

    fn from_address(address: usize) -> Self {
        address.try_into().ok().unwrap_or(Self::NULL)
    }

    fn decrease(&mut self) {
        *self -= Self::ONE
    }

    fn increase(&mut self) {
        *self += Self::ONE
    }
}

macro_rules! impl_addressable {
    ($($t:ty),*) => {
        $(
            impl Addressable for $t {
                const NULL: $t = !0;
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

    #[derive(Debug)]
    pub(super) struct DancingLinksBuilder<'a, I: Addressable> {
        dl: &'a mut DancingLinks<I>,
    }

    impl<'a, I: Addressable> DancingLinksBuilder<'a, I> {
        pub(super) fn new(dl: &'a mut DancingLinks<I>) -> Self {
            Self { dl }
        }

        fn grow<X: Default>(data: &mut Vec<X>, ix: I) -> &mut X {
            let ix = ix.address().unwrap();
            data.resize_with(data.len().max(ix), <_>::default);
            &mut data[ix]
        }

        fn insert_element(&mut self, hidx: I, cell_idx: I) -> I {
            let elem = Self::grow(&mut self.dl.elements, hidx);
            let old = elem.first;
            elem.first = cell_idx;
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
                prev_set: I::NULL,
                next_set,
                set,
                element,
            });
        }
    }
}
