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
        } else {}

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

trait Dimension: Copy {
    type Out<S, E>;
    fn dim<S, E>(la: impl FnOnce() -> S, lb: impl FnOnce() -> E) -> Self::Out<S, E>;
    fn dim_same<A>(la: impl FnOnce() -> A, lb: impl FnOnce() -> A) -> A;
    fn dimension<S, E>(a: S, b: E) -> Self::Out<S, E> {
        Self::dim(|| a, || b)
    }

    fn of<S, E>(self, a: S, b: E) -> Self::Out<S, E> {
        Self::dimension(a, b)
    }

    fn of_same<A>(self, a: A, b: A) -> A {
        Self::dim_same(|| a, || b)
    }
}

#[derive(Clone, Copy)]
struct Elements;

impl Dimension for Elements {
    type Out<S, E> = E;
    fn dim<S, E>(la: impl FnOnce() -> S, lb: impl FnOnce() -> E) -> Self::Out<S, E> {
        lb()
    }
    fn dim_same<A>(la: impl FnOnce() -> A, lb: impl FnOnce() -> A) -> A {
        lb()
    }
}

#[derive(Clone, Copy)]
struct Sets;

impl Dimension for Sets {
    type Out<S, E> = S;
    fn dim<S, E>(la: impl FnOnce() -> S, lb: impl FnOnce() -> E) -> Self::Out<S, E> {
        la()
    }
    fn dim_same<A>(la: impl FnOnce() -> A, lb: impl FnOnce() -> A) -> A {
        la()
    }
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

        fn dimension<'b, D: Dimension>(
            &'b mut self,
            d: D,
        ) -> D::Out<Getter<'b, S, I>, Getter<'b, E, I>> {
            D::dimension(
                Getter::new(&mut self.dl.sets, &mut self.set_map),
                Getter::new(&mut self.dl.elements, &mut self.elem_map),
            )
        }

        pub(super) fn add_link(&mut self, set: S, elem: E) {
            let i = I::from_address(self.dl.cells.len());
            let (set, next_set) = self.dimension(Sets).insert(set, i);
            let (element, next_element) = self.dimension(Elements).insert(elem, i);
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
