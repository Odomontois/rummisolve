struct ElementInfo<E, I> {
    element: E,
    sets: I,
    amount: I,
}

struct SetInfo<E, I> {
    set: E,
    elements: I,
}

struct DancingLinks<I: Addressable, S, E> {
    sets: Vec<SetInfo<S, I>>,
    elements: Vec<ElementInfo<E, I>>,
    cards: Vec<Card<I>>,
    backtrack: Vec<I>,
}

impl<I: Addressable, S, E> DancingLinks<I, S, E> {
    fn new(xs: impl IntoIterator<Item = (S, E)>) -> Self {
        todo!()
    }
}

struct Card<I: Addressable> {
    prev_element: I,
    next_element: I,
    prev_set: I,
    next_set: I,
    set: I,
    element: I,
    count: I,
}

trait Addressable: Copy + Eq + TryInto<usize> {
    const NULL: Self;
    fn address(self) -> Option<usize> {
        if self == Self::NULL {
            None
        } else {
            self.try_into().ok()
        }
    }
}

impl Addressable for u16 {
    const NULL: u16 = !0;
}

impl Addressable for u32 {
    const NULL: u32 = !0;
}

#[test]
#[ignore]
fn check() {
    println!("{}", std::mem::size_of::<Card<u16>>());
    println!("{}", std::mem::size_of::<Card<u32>>());
}
