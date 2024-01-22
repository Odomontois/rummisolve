pub trait RefMode<R> {
    type Ref<'a, T>;

    fn to_ref<'a, T>(r: Self::Ref<'a, T>) -> &'a T;
}
