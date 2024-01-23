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

    fn remove(&mut self, i: I) {}
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

trait Dimension<T> {
    type Data;
    type I: Addressable;
    type Links;
}

struct Elements;

impl<'a, I: Addressable, S: 'a, E: 'a> Dimension<(&'a (), I, S, E)> for Elements {
    type Data = E;
    type I = I;
    type Links = DancingLinks<I, S, E>;
}

struct Sets;

impl<'a, I: Addressable, S: 'a, E: 'a> Dimension<(&'a (), I, S, E)> for Sets {
    type Data = S;
    type I = I;
    type Links = DancingLinks<I, S, E>;
}

mod builder {
    use std::{collections::HashMap, hash::Hash};

    use super::*;
    trait BuilderDimension<T>: Dimension<T> {
        type Builder;
        fn mut_builder(self, builder: &mut Self::Builder) -> builder::Getter<Self::Data, Self::I>;
    }
    impl<'a, I: Addressable, S: 'a, E: 'a> BuilderDimension<(&'a (), I, S, E)> for Elements {
        type Builder = builder::DancingLinksBuilder<'a, I, S, E>;

        fn mut_builder(self, builder: &mut Self::Builder) -> builder::Getter<Self::Data, Self::I> {
            Getter {
                headers: &mut builder.dl.elements,
                map: &mut builder.elem_map,
            }
        }
    }

    impl<'a, I: Addressable, S: 'a, E: 'a> BuilderDimension<(&'a (), I, S, E)> for Sets {
        type Builder = builder::DancingLinksBuilder<'a, I, S, E>;

        fn mut_builder(self, builder: &mut Self::Builder) -> builder::Getter<Self::Data, Self::I> {
            Getter {
                headers: &mut builder.dl.sets,
                map: &mut builder.set_map,
            }
        }
    }

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

        fn dimension<D: BuilderDimension<(&'a (), I, S, E), Builder = Self>>(&mut self, d: D) -> Getter<D::Data, D::I>{
            d.mut_builder(self)
        } 

        fn sets_elems(&mut self) -> (Getter<S, I>, Getter<E, I>) {
            (
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
