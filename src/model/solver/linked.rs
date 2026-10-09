use std::ops::Add;

use derivative::Derivative;

use crate::model::solver::{Count, Ix};

use super::Address;

pub(crate) trait LinkedList<I: Address> {
    fn head(&mut self) -> &mut Option<I>;
    fn node(&mut self, ix: I) -> &mut LinkedNode<I>;

    fn delete(&mut self, i: I) {
        let node = *self.node(i);
        if let Some(prev) = node.prev {
            self.node(prev).next = node.next;
        } else {
            *self.head() = node.next;
        }
        if let Some(next) = node.next {
            self.node(next).prev = node.prev
        }
    }

    fn insert(&mut self, i: I) {
        let head = *self.head();
        let node = self.node(i);
        node.prev = None;
        node.next = head;
        *self.head() = Some(i)
    }

    fn reinsert(&mut self, i: I) {
        let node = *self.node(i);
        if let Some(next) = node.next {
            self.node(next).prev = Some(i)
        }
        if let Some(prev) = node.prev {
            self.node(prev).next = Some(i)
        } else {
            *self.head() = Some(i)
        }
    }
}

// impl <I: Address> LinkedList<I> for !{
//     fn head(&mut self) -> &mut Option<I> {
//         todo!()
//     }

//     fn node(&mut self, ix: I) -> &mut LinkedNode<I> {
//         todo!()
//     }
// }

#[derive(Derivative, Debug, Clone, Copy)]
#[derivative(Default(bound = ""))]
pub(crate) struct LinkedNode<I> {
    pub(crate) prev: Option<I>,
    pub(crate) next: Option<I>,
}

#[derive(Debug, Clone, Copy)]
struct ChooseMinCell<C, I> {
    count: C,
    node: LinkedNode<I>,
}
#[derive(Clone, Debug)]
pub(crate) struct ChooseMin<C, I> {
    min_level: I,
    levels: Vec<Option<I>>,
    list: Vec<ChooseMinCell<C, I>>,
}

impl<I: Address> ChooseMin<I::Count, I> {
    pub(crate) fn min(&self) -> Option<I> {
        self.levels[self.min_level.ix()]
    }

    fn cell_level(&self, elem: I) -> I {
        self.list[elem.ix()].count.address().unwrap()
    }

    fn list(&mut self, elem: I) -> impl LinkedList<I> + '_ {
        struct List<'a, I: Address>(&'a mut ChooseMin<I::Count, I>, I);

        impl<'a, I: Address> LinkedList<I> for List<'a, I> {
            fn head(&mut self) -> &mut Option<I> {
                let level = self.0.cell_level(self.1);
                &mut self.0.levels[level.ix()]
            }

            fn node(&mut self, ix: I) -> &mut LinkedNode<I> {
                &mut self.0.list[ix.ix()].node
            }
        }

        List(self, elem)
    }

    fn insert(&mut self, elem: I) {
        self.list(self.cell_level(elem)).insert(elem);
    }

    pub(crate) fn update_count(&mut self, elem: I, f: impl FnOnce(&mut I::Count)) {
        self.list(self.cell_level(elem)).delete(elem);
        f(&mut self.list[elem.ix()].count);
        let new_level = self.cell_level(elem);
        self.min_level = self.min_level.min(new_level);
        self.list(new_level).insert(elem);
    }
}

impl<I: Address> FromIterator<I::Count> for ChooseMin<I::Count, I> {
    fn from_iter<T: IntoIterator<Item = I::Count>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        let mut list = Vec::with_capacity(it.size_hint().0);
        let mut max_count = I::Count::ZERO;
        let mut min_count = None::<I>;
        for (i, count) in it.enumerate() {
            let ix = I::from_address(i);
            let node: LinkedNode<_> = <_>::default();
            list.push(ChooseMinCell { count, node });
            max_count = max_count.max(count);
            let count = count.address();
            if let (Some(min_count), Some(count)) = (&mut min_count, count){
                *min_count = count.min(*min_count);
            } else {
                min_count = count;
            }
        }
        let min_level = min_count.unwrap_or(I::ONE);
        let levels = vec![None; (max_count.try_into().unwrap_or(0))];

        let mut res = Self {
            list,
            min_level,
            levels,
        };

        for i in 0..res.list.len() {
            res.insert(I::from_address(i));
        }

        res
    }
}
