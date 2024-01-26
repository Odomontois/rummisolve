use crate::utils::hkt::{
    Dimension, First as Sets, First, Second as Elements, Second, TypeConstructor,
};
use std::{collections::HashMap, hash::Hash};

use derivative::Derivative;

#[derive(Default, Debug, Clone, Copy)]
struct Header<E, I> {
    value: E,
    first: I,
    amount: usize,
}

impl<'a, I: 'a> TypeConstructor<'a> for Header<(), I> {
    type Out<T: 'a> = Header<T, I>;
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Default(bound = ""))]
struct DancingLinks<I: Addressable, S, E> {
    sets: Vec<Header<S, I>>,
    elements: Vec<Header<E, I>>,
    cells: Vec<Cell<I>>,
    backstack: Vec<I>,
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

    fn remove(&mut self, i: I, d: impl Dimension) {
        let Some(&cur) = self.cell(i) else { return };
        if let Some(prev) = self.cell_mut(cur.prev(d)) {
            *prev.next_mut(d) = cur.next(d);
        } else {
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

trait Addressable: Copy + Eq + TryInto<usize> + TryFrom<usize> + 'static {
    const NULL: Self;
    fn address(self) -> Option<usize> {
        if self == Self::NULL {
            None
        } else {
            self.try_into().ok()
        }
    }

    fn from_address(address: usize) -> Self {
        address.try_into().ok().unwrap_or(Self::NULL)
    }
}

macro_rules! impl_addressable {
    ($($t:ty),*) => {
        $(
            impl Addressable for $t {
                const NULL: $t = !0;
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
            let headers = D::choose_val::<(&'b mut (), Vec<()>, Header<(), I>), _, _>(
                &mut self.dl.sets,
                &mut self.dl.elements,
            );

            let map = D::choose_val::<(&'b mut (), At<HashMap<(), usize>, First>), _, _>(
                &mut self.set_map,
                &mut self.elem_map,
            );

            let j = *map.entry(x.clone()).or_insert_with(|| {
                let i = headers.len();
                headers.push(Header {
                    value: x,
                    first: I::NULL,
                    amount: 0,
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
