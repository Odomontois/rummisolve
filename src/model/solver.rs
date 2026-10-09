#![allow(unused)]

mod address;
mod linked;
mod choose_min;

use std::{any::TypeId, collections::BTreeSet};

use derivative::Derivative;
use std::fmt::Debug;

pub(crate) use address::{Address, Count, Ix};
pub(crate) use linked::{LinkedList, LinkedNode};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ElementHeader<C, I> {
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

impl<I: Address> DancingLinks<I::Count, I> {
    fn new(xs: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let mut dl = Self::default();
        let mut builder = builder::DancingLinksBuilder::new(&mut dl);

        for (set, elem) in xs {
            builder.add_link(I::from_address(set), I::from_address(elem));
        }

        dl
    }

    fn on_element_change(&mut self, i: I, f: impl FnOnce(&mut I::Count)) {
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
        if elem.amount == I::Count::ZERO {
            let mut cur = elem.first;
            while let Some(i) = cur {
                let cell = self.cells[Ix(i)];
                self.remove_set(cell.set)?;
                cur = cell.set_list.next;
            }
        }
        DONE
    }

    fn set_list<'a>(&'a mut self, element: I) -> impl LinkedList<I> + 'a {
        struct SetList<'a, C, I>(&'a mut DancingLinks<C, I>, I);

        impl<'a, C, I: Address> LinkedList<I> for SetList<'a, C, I> {
            fn head(&mut self) -> &mut Option<I> {
                &mut self.0.elements[Ix(self.1)].first
            }

            fn node(&mut self, ix: I) -> &mut LinkedNode<I> {
                &mut self.0.cells[Ix(ix)].set_list
            }
        }
        SetList(self, element)
    }

    fn remove_cell(&mut self, i: I) -> Option<()> {
        let cell = self.cells[Ix(i)];
        self.backstack.push(Backstack::Cell(i));
        self.set_list(cell.set).delete(i);
        self.elements[Ix(i)].first?;
        self.on_element_change(i, I::Count::decrease);
        DONE
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Cell<I> {
    set_list: LinkedNode<I>,
    set: I,
    element: I,
}
mod builder {

    use super::*;

    #[derive(Derivative, Debug)]
    pub(super) struct DancingLinksBuilder<'a, C: Count<Idx = I>, I: Address> {
        dl: &'a mut DancingLinks<C, C::Idx>,
    }

    impl<'a, I: Address> DancingLinksBuilder<'a, I::Count, I> {
        pub(super) fn new(dl: &'a mut DancingLinks<I::Count, I>) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::model::solver::{DancingLinks, DancingLinksOf};

    #[test]
    fn check() {
        let u: DancingLinksOf<u8> = DancingLinks::new([]);
    }
}

fn lol<A: 'static>() -> Vec<A> {
    println!("{:?}", TypeId::of::<A>());
    vec![]
}

#[test]
fn lols() {
    lol::<Vec<Option<String>>>();
    lol::<Vec<Option<String>>>();
    lol::<([&'static u64; 4])>();
}

