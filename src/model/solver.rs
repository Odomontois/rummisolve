use std::{collections::HashMap, default, hash::Hash};

use derivative::Derivative;

#[derive(Default, Debug, Clone, Copy)]
struct ElementInfo<E, I> {
    element: E,
    sets: I,
    amount: usize,
}

#[derive(Default, Debug, Clone, Copy)]
struct SetInfo<E, I> {
    set: E,
    elements: I,
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Default(bound = ""))]
struct DancingLinks<I: Addressable, S, E> {
    sets: Vec<SetInfo<S, I>>,
    elements: Vec<ElementInfo<E, I>>,
    cards: Vec<Card<I>>,
    backstack: Vec<I>,
}

impl<I: Addressable, S, E> DancingLinks<I, S, E> {
    fn new(xs: impl IntoIterator<Item = (S, E)>, pool: impl IntoIterator<Item = (E, usize)>) -> Self
    where
        S: Clone + Hash + Eq,
        E: Clone + Hash + Eq,
    {
        let mut builder = DancingLinksBuilder::default();

        for (elem, count) in pool {
            builder.add_elem(elem, count);
        }

        for (set, elem) in xs {
            builder.add_link(set, elem);
        }

        builder.dl
    }

    // fn add
}

#[derive(Default, Debug, Clone, Copy)]
struct Card<I: Addressable> {
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

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(bound = ""))]
struct DancingLinksBuilder<I: Addressable, S, E> {
    dl: DancingLinks<I, S, E>,
    elem_map: HashMap<E, usize>,
    set_map: HashMap<S, usize>,
}

impl<I: Addressable, S: Eq + Hash + Clone, E: Eq + Hash + Clone> DancingLinksBuilder<I, S, E> {
    fn get_elem(&mut self, elem: E) -> usize {
        let i = *self.elem_map.entry(elem.clone()).or_insert_with(|| {
            let i = self.dl.elements.len();
            self.dl.elements.push(ElementInfo {
                element: elem,
                sets: I::NULL,
                amount: 0,
            });
            i
        });
        i
    }

    fn add_elem(&mut self, elem: E, count: usize) {
        let i = self.get_elem(elem);
        self.dl.elements[i].amount += count;
    }

    fn get_state(&mut self, state: S) -> usize {
        let i = *self.set_map.entry(state.clone()).or_insert_with(|| {
            let i = self.dl.sets.len();
            self.dl.sets.push(SetInfo {
                set: state,
                elements: I::NULL,
            });
            i
        });
        i
    }

    fn add_link(&mut self, state: S, elem: E) {
        let si = self.get_state(state);
        let ei = self.get_elem(elem);
        let card = Card {
            prev_element: I::NULL,
            next_element: self.dl.sets[si].elements,
            prev_set: I::NULL,
            next_set: self.dl.elements[ei].sets,
            set: I::from_address(si),
            element: I::from_address(ei),
        };
        let i = I::from_address(self.dl.cards.len());
        self.dl.cards.push(card);
        self.dl.sets[si].elements = i;
        self.dl.elements[ei].sets = i;
    }
}

#[test]
#[ignore]
fn check() {
    println!("{}", std::mem::size_of::<Card<u16>>());
    println!("{}", std::mem::size_of::<Card<u32>>());
}
