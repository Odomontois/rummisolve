trait SliceUtils {
    type Item;
    fn divide_by(&mut self, p: impl Fn(&Self::Item) -> bool) -> usize;
}

impl<T> SliceUtils for [T] {
    type Item = T;
    fn divide_by(&mut self, p: impl Fn(&Self::Item) -> bool) -> usize {
        let mut i = 0;
        let mut j = 0;
        while j < self.len() {
            if p(&self[j]) {
                self.swap(i, j);
                i += 1;
            }
            j += 1;
        }
        i
    }
}
