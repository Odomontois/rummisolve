use std::{collections::HashMap, marker::PhantomData};

pub(crate) trait TypeConstructor<'a> {
    type Out<T: 'a>: 'a;
}

impl<'a, X: TypeConstructor<'a>> TypeConstructor<'a> for Option<X> {
    type Out<T: 'a> = Option<X::Out<T>>;
}

impl<'a, X: 'a> TypeConstructor<'a> for (X,) {
    type Out<T: 'a> = X;
}

impl<'a> TypeConstructor<'a> for () {
    type Out<T: 'a> = T;
}

impl<'a, X: TypeConstructor<'a>, Y: TypeConstructor<'a>> TypeConstructor<'a> for (X, Y) {
    type Out<T: 'a> = X::Out<Y::Out<T>>;
}

impl<'a, X: TypeConstructor<'a>, Y: TypeConstructor<'a>, Z: TypeConstructor<'a>> TypeConstructor<'a>
    for (X, Y, Z)
{
    type Out<T: 'a> = X::Out<Y::Out<Z::Out<T>>>;
}

impl<'a, X: TypeConstructor<'a>> TypeConstructor<'a> for Vec<X> {
    type Out<T: 'a> = Vec<X::Out<T>>;
}

impl<'a, X: TypeConstructor<'a>> TypeConstructor<'a> for &'a X {
    type Out<T: 'a> = &'a X::Out<T>;
}

impl<'a, X: TypeConstructor<'a>> TypeConstructor<'a> for &'a mut X {
    type Out<T: 'a> = &'a mut X::Out<T>;
}

pub(crate) trait Dimension: Copy {
    type Out<'a, S: 'a, E: 'a>: 'a;

    fn choose_mut<'a, F: TypeConstructor<'a>, A, L: 'a, R: 'a>(
        self,
        a: &mut A,
        ll: impl FnOnce(&mut A) -> F::Out<L>,
        lr: impl FnOnce(&mut A) -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>>;

    fn choose<'a, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        self,
        ll: impl FnOnce() -> F::Out<L>,
        lr: impl FnOnce() -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        self.choose_mut::<F, (), L, R>(&mut (), |_| ll(), |_| lr())
    }

    fn choose_val<'a, F: TypeConstructor<'a>, L: 'a, R: 'a>(
        self,
        ll: F::Out<L>,
        lr: F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        self.choose::<F, L, R>(|| ll, || lr)
    }

    fn of_same<'a, X: 'a>(self, a: X, b: X) -> X {
        self.choose_val::<(X,), (), ()>(a, b)
    }
}

pub(crate) struct At<A, X>(PhantomData<(A, X)>);

impl<'a, X: TypeConstructor<'a>, V: 'a> TypeConstructor<'a> for At<HashMap<X, V>, First> {
    type Out<K: 'a> = HashMap<X::Out<K>, V>;
}

impl<'a, K: 'a, X: TypeConstructor<'a>> TypeConstructor<'a> for At<HashMap<K, X>, Second> {
    type Out<V: 'a> = HashMap<K, X::Out<V>>;
}

#[derive(Clone, Copy)]
pub(crate) struct First;
impl Dimension for First {
    type Out<'a, S: 'a, E: 'a> = S;

    fn choose_mut<'a, 'b, F: TypeConstructor<'a>, A, L: 'a, R: 'a>(
        self,
        a: &mut A,
        ll: impl FnOnce(&mut A) -> F::Out<L>,
        _lr: impl FnOnce(&mut A) -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        ll(a)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Second;
impl Dimension for Second {
    type Out<'a, S: 'a, E: 'a> = E;

    fn choose_mut<'a, 'b, F: TypeConstructor<'a>, A, L: 'a, R: 'a>(
        self,
        a: &mut A,
        _ll: impl FnOnce(&mut A) -> F::Out<L>,
        lr: impl FnOnce(&mut A) -> F::Out<R>,
    ) -> F::Out<Self::Out<'a, L, R>> {
        lr(a)
    }
}

#[test]
fn check_size() {
    assert_eq!(size_of_val(&()), 0)
}
