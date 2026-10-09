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

        self.list(new_level).insert(elem);

        if let Some(min_level) = &mut self.min_level {
            *min_level = new_level.min(*min_level)
        } else {
            self.min_level = Some(new_level)
        }

        if let Some(min_level) = &mut self.min_level {
            let mut level = min_level.address();
            while self.levels[level].is_none() {
                level += 1;
            }
            *min_level = I::from_address(level);
        }
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
mod tests;
