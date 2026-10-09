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

