use super::{Address, Count, LinkedList, LinkedNode};

#[derive(Debug, Clone, Copy)]
struct ChooseMinCell<C, I> {
    count: C,
    node: LinkedNode<I>,
}

#[derive(Clone, Debug)]
pub(crate) struct ChooseMin<C, I> {
    min_level: Option<I>,
    levels: Vec<Option<I>>,
    list: Vec<ChooseMinCell<C, I>>,
}

impl<I: Address> ChooseMin<I::Count, I> {
    pub(crate) fn min(&self) -> Option<I> {
        self.levels[self.min_level?.ix()]
    }

    fn cell_level(&self, elem: I) -> I {
        self.list[elem.ix()].count.address().unwrap()
    }

    fn list(&mut self, level: I) -> impl LinkedList<I> + '_ {
        struct List<'a, I: Address>(&'a mut ChooseMin<I::Count, I>, I);

        impl<'a, I: Address> LinkedList<I> for List<'a, I> {
            fn head(&mut self) -> &mut Option<I> {
                &mut self.0.levels[self.1.ix()]
            }

            fn node(&mut self, ix: I) -> &mut LinkedNode<I> {
                &mut self.0.list[ix.ix()].node
            }
        }

        List(self, level)
    }

    fn insert(&mut self, elem: I) {
        self.list(self.cell_level(elem)).insert(elem);
    }

    pub(crate) fn update_count(&mut self, elem: I, f: impl FnOnce(&mut I::Count)) {
        self.list(self.cell_level(elem)).delete(elem);
        f(&mut self.list[elem.ix()].count);
        let new_level = self.cell_level(elem);
        if let Some(min_level) = &mut self.min_level {
            *min_level = new_level.min(*min_level)
        } else {
            self.min_level = Some(new_level)
        }
        self.list(new_level).insert(elem);
    }
}

impl<I: Address> FromIterator<I::Count> for ChooseMin<I::Count, I> {
    fn from_iter<T: IntoIterator<Item = I::Count>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        let mut list = Vec::with_capacity(it.size_hint().0);
        let mut max_count = I::Count::ZERO;
        let mut min_level = None::<I>;
        for (i, count) in it.enumerate() {
            let ix = I::from_address(i);
            let node: LinkedNode<_> = <_>::default();
            list.push(ChooseMinCell { count, node });
            max_count = max_count.max(count);
            let count = count.address();
            if let (Some(min_count), Some(count)) = (&mut min_level, count) {
                *min_count = count.min(*min_count);
            } else {
                min_level = count;
            }
        }
        let levels = vec![None; (max_count.try_into().unwrap_or(0) + 1)];

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

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use super::*;

    type I = NonZero<u8>;
    type CM = ChooseMin<u8, I>;

    fn ix(i: usize) -> I {
        I::from_address(i)
    }

    fn check_invariants(cm: &CM) {
        let mut seen = vec![false; cm.list.len()];
        for (level, head) in cm.levels.iter().enumerate() {
            let mut prev = None;
            let mut cur = *head;
            while let Some(e) = cur {
                let cell = &cm.list[e.ix()];
                assert_eq!(cell.count as usize, level, "element {e} in wrong level");
                assert_eq!(cell.node.prev, prev, "broken prev link at {e}");
                assert!(!seen[e.address()], "element {e} seen twice");
                seen[e.address()] = true;
                prev = cur;
                cur = cell.node.next;
            }
        }
        let missing: Vec<_> = (0..seen.len()).filter(|&i| !seen[i]).collect();
        assert!(missing.is_empty(), "elements not in any level: {missing:?}");
    }

    fn check_min(cm: &CM, counts: &[u8]) {
        match (cm.min(), counts.iter().min()) {
            (None, None) => {}
            (Some(e), Some(&m)) => {
                assert_eq!(counts[e.address()], m, "min() = {e}, counts = {counts:?}")
            }
            (got, expected) => panic!("min() = {got:?}, expected count {expected:?}"),
        }
    }

    fn build(counts: &[u8]) -> CM {
        let cm: CM = counts.iter().copied().collect();
        println!("{cm:?}");
        check_invariants(&cm);
        check_min(&cm, counts);
        cm
    }

    #[test]
    fn empty_has_no_min() {
        build(&[]);
    }

    #[test]
    fn single_element() {
        let cm = build(&[3]);
        assert_eq!(cm.min(), Some(ix(0)));
    }

    #[test]
    fn picks_minimum() {
        let cm = build(&[3, 1, 2]);
        assert_eq!(cm.min(), Some(ix(1)));
    }

    #[test]
    fn picks_zero_count() {
        let cm = build(&[2, 0, 1]);
        assert_eq!(cm.min(), Some(ix(1)));
    }

    #[test]
    fn ties_pick_any_minimum() {
        build(&[2, 1, 3, 1]);
    }

    #[test]
    fn decrease_moves_to_new_min() {
        let mut counts = [3, 2, 3];
        let mut cm = build(&counts);
        for _ in 0..2 {
            cm.update_count(ix(0), |c| c.decrease());
            counts[0] -= 1;
            check_invariants(&cm);
            check_min(&cm, &counts);
        }
        assert_eq!(cm.min(), Some(ix(0)));
    }

    #[test]
    fn increase_after_decrease_restores_min() {
        // cover / uncover pattern from dancing links
        let mut cm = build(&[2, 3, 2]);
        cm.update_count(ix(0), |c| c.decrease());
        check_min(&cm, &[1, 3, 2]);
        cm.update_count(ix(0), |c| c.increase());
        check_invariants(&cm);
        check_min(&cm, &[2, 3, 2]);
    }

    #[test]
    fn increase_only_min_element() {
        let mut cm = build(&[1, 2, 3]);
        cm.update_count(ix(0), |c| c.increase());
        check_invariants(&cm);
        check_min(&cm, &[2, 2, 3]);
    }

    #[test]
    fn random_updates_keep_invariants() {
        let mut seed: u64 = 0x2545_F491_4F6C_DD1D;
        let mut next = move |n: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) % n
        };
        for _ in 0..50 {
            let len = 1 + next(12) as usize;
            let mut counts: Vec<u8> = (0..len).map(|_| 1 + next(6) as u8).collect();
            let max = *counts.iter().max().unwrap();
            let mut cm = build(&counts);
            for _ in 0..200 {
                let e = next(len as u64) as usize;
                let up = counts[e] == 0 || (counts[e] < max && next(2) == 0);
                if up {
                    cm.update_count(ix(e), |c| c.increase());
                    counts[e] += 1;
                } else {
                    cm.update_count(ix(e), |c| c.decrease());
                    counts[e] -= 1;
                }
                check_invariants(&cm);
                check_min(&cm, &counts);
            }
        }
    }
}
