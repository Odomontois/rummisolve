use crate::utils::hkt::{
    Dimension, First as Sets, First, Second as Elements, Second, TypeConstructor,
};
use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    ops::{Add, AddAssign, SubAssign},
    slice::SliceIndex,
};

use derivative::Derivative;
use std::ops::Index;

#[derive(Default, Debug, Clone, Copy)]
struct Header<E, I> {
    value: E,
    first: I,
    amount: I,
}

impl<'a, X: TypeConstructor<'a>, I: 'a> TypeConstructor<'a> for Header<X, I> {
    type Out<T: 'a> = Header<X::Out<T>, I>;
}

#[derive(Debug, Clone, Copy)]
enum Backstack<I> {
    Cell(I),
    Set(I),
    Element(I),
    Chosen { cell: I },
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Default(bound = ""))]
struct Headers<I: Addressable, S, E> {
    sets: Vec<Header<S, I>>,
    elements: Vec<Header<E, I>>,
}

impl<I: Addressable, S, E> Headers<I, S, E> {
    fn dim_mut<D: Dimension>(&mut self, d: D) -> &mut Vec<Header<D::Out<'_, S, E>, I>> {
        d.choose_val::<&mut Vec<Header<(), I>>, S, E>(&mut self.sets, &mut self.elements)
    }
    fn get_mut<D: Dimension>(&mut self, i: I, d: D) -> Option<&mut Header<D::Out<'_, S, E>, I>> {
        i.get_mut_from(self.dim_mut(d))
    }
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Default(bound = ""))]
struct DancingLinks<I: Addressable, S, E> {
    headers: Headers<I, S, E>,
    cells: Vec<Cell<I>>,
    backstack: Vec<Backstack<I>>,
    element_choose: BTreeSet<(I, I)>,
}

impl<I: Addressable, S, E> DancingLinks<I, S, E> {
    fn new(xs: impl IntoIterator<Item = (S, E)>, pool: impl IntoIterator<Item = (E, usize)>) -> Self
    where
        S: Clone + Hash + Eq,
        E: Clone + Hash + Eq,
    {
        let mut dl = Self::default();
        let mut builder = builder::DancingLinksBuilder::new(&mut dl);

        for (set, elem) in xs {
            builder.add_link(set, elem);
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
        let hd = i.get_mut_from(&mut self.headers.sets).unwrap();
        self.element_choose.remove(&(hd.amount, i));
        f(&mut hd.amount);
        self.element_choose.insert((hd.amount, i));
    }

    fn on_set_change(&mut self, i: I, f: impl FnOnce(&mut I)) {
        f(&mut i.get_mut_from(&mut self.headers.sets).unwrap().amount);
    }

    fn on_dim_change(&mut self, d: impl Dimension, i: I, f: impl FnOnce(&mut I) + Copy) {
        d.choose_mut::<(), Self, (), ()>(
            self,
            |s: &mut Self| s.on_set_change(i, f),
            |s: &mut Self| s.on_element_change(i, f),
        );
    }

    fn remove_cell_dim(&mut self, i: I, d: impl Dimension) {
        let Some(&cur) = self.cell(i) else { return };
        let hi = cur.header_idx(d);
        let hd = self.headers.get_mut(hi, d).unwrap();
        debug_assert!(hd.first == i);
        if let Some(prev) = cur.prev(d).get_mut_from(&mut self.cells) {
            *prev.next_mut(d) = cur.next(d);
        } else {
            hd.first = cur.next(d);
        }
        self.on_dim_change(d, hi, I::decrease);
    }

    fn remove_cell(&mut self, i: I) {
        self.remove_cell_dim(i, Sets);
        self.backstack.push(Backstack::Cell(i));
    }

    fn remove_set(&mut self, i: I) {
        let mut cell = i.get_from(&self.headers.sets).unwrap().first;
        while let Some(c) = cell.get_from(&self.cells) {
            let prev = cell;
            cell = c.next_element;
            self.remove_cell(prev);
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Cell<I: Addressable> {
    prev_element: I,
    next_element: I,
    prev_set: I,
    next_set: I,
    set: I,
    element: I,
}

impl<I: Addressable> Cell<I> {
    fn header_idx(&self, d: impl Dimension) -> I {
        d.of_same(self.set, self.element)
    }

    fn prev(&self, d: impl Dimension) -> I {
        d.of_same(self.prev_set, self.prev_element)
    }

    fn prev_mut(&mut self, d: impl Dimension) -> &mut I {
        d.of_same(&mut self.prev_set, &mut self.prev_element)
    }

    fn next(&self, d: impl Dimension) -> I {
        d.of_same(self.next_set, self.next_element)
    }

    fn next_mut(&mut self, d: impl Dimension) -> &mut I {
        d.of_same(&mut self.next_set, &mut self.next_element)
    }
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
    use std::{collections::HashMap, hash::Hash};

    use crate::utils::hkt::{At, TypeConstructor};

    use super::*;

    #[derive(Debug)]
    pub(super) struct DancingLinksBuilder<'a, I: Addressable, S, E> {
        dl: &'a mut DancingLinks<I, S, E>,
        elem_map: HashMap<E, usize>,
        set_map: HashMap<S, usize>,
    }

    impl<'a, I: Addressable, S: Eq + Hash + Clone, E: Eq + Hash + Clone>
        DancingLinksBuilder<'a, I, S, E>
    {
        pub(super) fn new(dl: &'a mut DancingLinks<I, S, E>) -> Self {
            Self {
                dl,
                elem_map: HashMap::new(),
                set_map: HashMap::new(),
            }
        }

        fn insert<'b, D: Dimension>(&'b mut self, d: D, x: D::Out<'b, S, E>, new: I) -> (I, I)
        where
            D::Out<'b, S, E>: Clone + Hash + Eq,
        {
            type HeaderHK<'b, I> = (&'b mut (), Vec<()>, Header<(), I>);
            let headers = self.dl.headers.dim_mut(d);

            type MapHK<'b> = (&'b mut (), At<HashMap<(), usize>, First>);
            let map = d.val::<MapHK, _, _>(&mut self.set_map, &mut self.elem_map);

            let j = *map.entry(x.clone()).or_insert_with(|| {
                let i = headers.len();
                headers.push(Header {
                    value: x,
                    first: I::NULL,
                    amount: I::ZERO,
                });
                i
            });

            headers[j].first = new;

            let old = headers[j].first;

            (I::from_address(j), old)
        }

        pub(super) fn add_link(&mut self, set: S, elem: E) {
            let i = I::from_address(self.dl.cells.len());
            let (set, next_set) = self.insert(Sets, set, i);
            let (element, next_element) = self.insert(Elements, elem, i);
            let (prev_element, prev_set) = (I::NULL, I::NULL);

            self.dl.cells.push(Cell {
                prev_element,
                next_element,
                prev_set,
                next_set,
                set,
                element,
            });
        }
    }
}
