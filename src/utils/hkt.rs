use std::marker::PhantomData;

pub(crate) trait TypeConstructor<'a> {
    type Out<T: 'a>: 'a;
}

pub(crate) struct Const<X>(PhantomData<X>);

impl<'a, X: 'a> TypeConstructor<'a> for Const<X> {
    type Out<T: 'a> = X;
}

pub(crate) struct Id;

impl<'a> TypeConstructor<'a> for Id {
    type Out<T: 'a> = T;
}

pub(crate) struct Compose<F, G>(PhantomData<(F, G)>);

impl<'a, F: TypeConstructor<'a>, G: TypeConstructor<'a>> TypeConstructor<'a> for Compose<F, G> {
    type Out<T: 'a> = F::Out<G::Out<T>>;
}

pub(crate) trait Dimension {
    type Out<'a, S: 'a, E: 'a>: 'a;

    fn choose<'a, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        ll: impl FnOnce() -> F::Out<L>,
        lr: impl FnOnce() -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>>;

    fn of<'a, F: TypeConstructor<'a>, L: 'a, R: 'a>(&self, a: L, b: R) -> Self::Out<'a, L, R> {
        Self::choose::<Id, _, _>(|| a, || b)
    }
}

pub(crate) struct First;
impl Dimension for First {
    type Out<'a, S: 'a, E: 'a> = S;

    fn choose<'a, 'b, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        ll: impl FnOnce() -> F::Out<L>,
        _lr: impl FnOnce() -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        ll()
    }
}

pub(crate) struct Second;
impl Dimension for Second {
    type Out<'a, S: 'a, E: 'a> = E;

    fn choose<'a, 'b, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        _ll: impl FnOnce() -> F::Out<L>,
        lr: impl FnOnce() -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        lr()
    }
}
