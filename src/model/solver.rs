use std::{
    any::TypeId, collections::{BTreeSet, HashMap}, hash::Hash, mem::take, ops::{Add, AddAssign, IndexMut, SubAssign}, slice::SliceIndex,
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

impl<C: Count, I> Default for ElementHeader<C, I> {
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

type DancingLinksOf<C> = DancingLinks<C, <C as Count>::Idx>;

impl<C: Count<Idx = I>, I: Address> DancingLinks<C, I> {
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

    fn remove_element(&mut self, elem_ix: I) -> Option<()> {
        let elem = &mut self.elements[Ix(elem_ix)];
        elem.amount.decrease();
        self.backstack.push(Backstack::Element(elem_ix));
        if elem.amount == C::ZERO {
            let mut cur = elem.first;
            while let Some(i) = cur {
                let cell = self.cells[Ix(i)];
                self.remove_set(cell.set)?;
                cur = cell.set_list.next;
            }
        }
        DONE
    }

    fn remove_cell(&mut self, i: I) -> Option<()> {
        // let &cur = &self.cells[Ix(i)];
        // let hi = cur.element.address();
        // let hd = &mut self.elements[hi];
        // debug_assert!(hd.first == Some(i));
        // if let Some(prev_set) = cur.prev_set {
        //     self.cells[Ix(prev_set)].next_set = cur.next_set;
        // } else {
        //     hd.first = cur.next_set;
        // }
        // if let Some(next_set) = cur.next_set {
        //     self.cells[Ix(next_set)].prev_set = cur.prev_set
        // }
        // if hd.first == None {
        //     return FAIL;
        // }
        // self.on_element_change(i, C::decrease);
        // self.backstack.push(Backstack::Cell(i));
        DONE
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct LinkedNode<I> {
    prev: Option<I>,
    next: Option<I>,
}
#[derive(Default, Debug, Clone, Copy)]
struct Cell<I> {
    set_list: LinkedNode<I>,
    set: I,
    element: I,
}

trait Count: Copy + Eq + Ord + SubAssign + AddAssign + 'static {
    type Idx: Address;
    const ONE: Self;
    const ZERO: Self;
    fn decrease(&mut self) {
        *self -= Self::ONE
    }

    fn increase(&mut self) {
        *self += Self::ONE
    }
}

trait Address: Copy + Eq + Ord + TryFrom<NonZero<usize>> + TryInto<NonZero<usize>> + 'static {
    type Count: Count;
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

mod builder {
    use std::{cell, collections::HashMap, hash::Hash};

    use super::*;

    #[derive(Derivative, Debug)]
    pub(super) struct DancingLinksBuilder<'a, C: Count<Idx = I>, I: Address> {
        dl: &'a mut DancingLinks<C, C::Idx>,
    }

    impl<'a, C: Count<Idx = I>, I: Address> DancingLinksBuilder<'a, C, I> {
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
            self.insert_set(set, i);
            let next = self.insert_element(element, i);
            let set_list = LinkedNode { prev: None, next };

            self.dl.cells.push(Cell {
                set_list,
                set,
                element,
            });
        }
    }
}

struct Linked<'a, I, F> {
    head: &'a mut Option<I>,
    list_f: F,
}

impl<'a, I: Address, F: Fn(I) -> &'a mut LinkedNode<I>> Linked<'a, I, F> {
    fn delete(self, i: I){
        // let &cur = &self.pool[Ix(i)];
        // debug_assert!(*self.head == Some(i));
        // if let Some(prev) = cur.prev_set {
        //     self.cells[Ix(prev)].next_set = cur.next_set;
        // } else {
        //     hd.first = cur.next_set;
        // }
        // if let Some(next_set) = cur.next_set {
        //     self.cells[Ix(next_set)].prev_set = cur.prev_set
        // }
        // if hd.first == None {
        //     return FAIL;
        // }
        // self.on_element_change(i, C::decrease);
        // self.backstack.push(Backstack::Cell(i));
    }
}

mod tests {
    use crate::model::solver::{DancingLinks, DancingLinksOf};

    #[test]
    fn check() {
        let u: DancingLinksOf<u8> = DancingLinks::new([]);
    }
}


fn lol<A: 'static>() -> Vec<A>{
    println!("{:?}", TypeId::of::<A>());
    vec![]
}

#[test]
fn lols(){
    lol::<Vec<Option<String>>>();
    lol::<Vec<Option<String>>>();
    lol::<([&'static u64; 4])>();
}