use std::{collections::HashMap, hash::Hash};

use derivative::Derivative;

#[derive(Default, Debug, Clone, Copy)]
struct Header<E, I> {
    value: E,
    first: I,
    amount: usize,
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

trait Addressable: Copy + Eq + TryInto<usize> + TryFrom<usize> {
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

impl Addressable for u16 {
    const NULL: u16 = !0;
}

impl Addressable for u32 {
    const NULL: u32 = !0;
}

mod builder {
    use std::{collections::HashMap, hash::Hash};

    use super::*;

    #[derive(Debug)]
    pub(super) struct DancingLinksBuilder<'a, I: Addressable, S, E> {
        dl: &'a mut DancingLinks<I, S, E>,
        elem_map: HashMap<E, usize>,
        set_map: HashMap<S, usize>,
    }

    struct Getter<'a, X, I> {
        headers: &'a mut Vec<Header<X, I>>,
        map: &'a mut HashMap<X, usize>,
    }

    impl<'a, X, I: Addressable> Getter<'a, X, I> {
        fn new(headers: &'a mut Vec<Header<X, I>>, map: &'a mut HashMap<X, usize>) -> Self {
            Self { headers, map }
        }

        fn insert(&mut self, x: X, new: I) -> (I, I)
        where
            X: Clone + Hash + Eq,
        {
            let j = *self.map.entry(x.clone()).or_insert_with(|| {
                let i = self.headers.len();
                self.headers.push(Header {
                    value: x,
                    first: I::NULL,
                    amount: 0,
                });
                i
            });
            let old = self.headers[j].first;
            self.headers[j].first = new;
            (I::from_address(j), old)
        }
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

        fn sets_elems(&mut self) -> (Getter<S, I>, Getter<E, I>) {
            (
                Getter::new(&mut self.dl.sets, &mut self.set_map),
                Getter::new(&mut self.dl.elements, &mut self.elem_map),
            )
        }

        pub(super) fn add_link(&mut self, set: S, elem: E) {
            let i = I::from_address(self.dl.cells.len());
            let (mut sets, mut elems) = self.sets_elems();
            let (set, next_set) = sets.insert(set, i);
            let (element, next_element) = elems.insert(elem, i);
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
