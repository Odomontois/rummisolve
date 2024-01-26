use std::{collections::HashMap, marker::PhantomData};

pub(crate) trait TypeConstructor<'a> {
    type Out<T: 'a>: 'a;
}

impl<'a, X: 'a> TypeConstructor<'a> for (X,) {
    type Out<T: 'a> = X;
}

impl<'a> TypeConstructor<'a> for () {
    type Out<T: 'a> = T;
}

impl<'a, F: TypeConstructor<'a>, G: TypeConstructor<'a>> TypeConstructor<'a> for (F, G) {
    type Out<T: 'a> = F::Out<G::Out<T>>;
}

impl<'a, F: TypeConstructor<'a>, G: TypeConstructor<'a>, H: TypeConstructor<'a>> TypeConstructor<'a>
    for (F, G, H)
{
    type Out<T: 'a> = F::Out<G::Out<H::Out<T>>>;
}

impl<'a> TypeConstructor<'a> for Vec<()> {
    type Out<T: 'a> = Vec<T>;
}

impl<'a> TypeConstructor<'a> for &'a () {
    type Out<T: 'a> = &'a T;
}

impl<'a> TypeConstructor<'a> for &'a mut () {
    type Out<T: 'a> = &'a mut T;
}

pub(crate) trait Dimension: Copy {
    type Out<'a, S: 'a, E: 'a>: 'a;

    fn choose<'a, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        ll: impl FnOnce() -> F::Out<L>,
        lr: impl FnOnce() -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>>;

    fn choose_val<'a, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        ll: F::Out<L>,
        lr: F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        Self::choose::<F, L, R>(|| ll, || lr)
    }

    fn val<'a, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        self,
        a: F::Out<L>,
        b: F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        Self::choose_val::<F, L, R>(a, b)
    }

    fn of<'a, L: 'a, R: 'a>(&self, a: L, b: R) -> Self::Out<'a, L, R> {
        Self::choose_val::<(), _, _>(a, b)
    }

    fn of_same<'a, X: 'a>(&self, a: X, b: X) -> X {
        Self::choose_val::<(X,), (), ()>(a, b)
    }
}

pub(crate) struct At<A, X>(PhantomData<(A, X)>);

impl<'a, V: 'a> TypeConstructor<'a> for At<HashMap<(), V>, First> {
    type Out<K: 'a> = HashMap<K, V>;
}

impl<'a, K: 'a> TypeConstructor<'a> for At<HashMap<K, ()>, Second> {
    type Out<V: 'a> = HashMap<K, V>;
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
