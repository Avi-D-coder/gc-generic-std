#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(marker_trait_attr)]

use generic_std::plug::*;
use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}

/// Realy `Gc<'r, T>(&'r T<'r>);`
pub struct Gc<'r, T>(&'r T);
impl<'r, T> Copy for Gc<'r, T> {}
impl<'r, T> Clone for Gc<'r, T> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct H2Gc;
pub struct H1Gc<'a>(PhantomData<&'a ()>);
impl<'a> PlugLifetime<'a> for H2Gc {
    type T = H1Gc<'a>;
}
impl<'a, T: 'a> PlugType<T> for H1Gc<'a> {
    type T = Gc<'a, <HTry<T> as PlugLifetime<'a>>::T>;
}
impl<'a, T> PlugLifetime<'a> for Gc<'a, T> {
    type T = Gc<'a, <HTry<T> as PlugLifetime<'a>>::T>;
}
impl<'b, A, B: 'b> PlugType<B> for Gc<'b, A> {
    type T = Gc<'b, B>;
}

pub struct HTry<T>(PhantomData<T>);
impl<'a, T> PlugLifetime<'a> for HTry<T> {
    default type T = T;
}
impl<'a, T: PlugLifetime<'a>> PlugLifetime<'a> for HTry<T> {
    type T = <T as PlugLifetime<'a>>::T;
}

fn transmute_lifetime<'b, A>(a: A) -> <HTry<A> as PlugLifetime<'b>>::T {
    todo!()
}

#[test]
fn unify_test() {
    fn foo<A, B: Id<A>>() {}
    foo::<usize, usize>();
    foo::<Gc<usize>, Gc<usize>>();

    fn lifes<'a, 'b, T: for<'l> PlugLifetime<'l>>()
    where
        <T as PlugLifetime<'a>>::T: PlugLifetime<'b>,
        <T as PlugLifetime<'b>>::T: Id<<<T as PlugLifetime<'a>>::T as PlugLifetime<'b>>::T>,
    {
        // let a: Gc<'a, usize> = Gc(&1);
        // let b: Gc<'b, usize> = transmute_lifetime(a);
        foo::<<<T as PlugLifetime<'a>>::T as PlugLifetime<'b>>::T, <T as PlugLifetime<'b>>::T>();
        // foo::<Gc<'a, usize>, Gc<'a, Ty<'a, usize>>>();
    }
    // foo::<Gc<usize>, Gc<Ty<Ty<String>>>>();
}

pub unsafe trait Id<T> {}
unsafe impl<T> Id<T> for T {}

#[marker]
pub unsafe trait TyEq<B> {}
// unsafe impl<T> TyEq<T> for T {}
unsafe impl<'a, A: PlugLifetime<'a>, B: PlugLifetime<'a>> TyEq<A> for B where A::T: Id<B::T> {}
unsafe impl<'a, A, B> TyEq<A> for B where
    <HTry<A> as PlugLifetime<'a>>::T: Id<<HTry<B> as PlugLifetime<'a>>::T>
{
}

pub trait Trace {}

type Ty<'r, T> = <HTry<T> as PlugLifetime<'r>>::T;
type Static<T> = <HTry<T> as PlugLifetime<'static>>::T;

pub struct Arena<A>(Vec<A>);

mod list {
    use super::*;

    #[derive(Clone)]
    pub struct List<'r, T>(Option<Gc<'r, Elem<'r, T>>>);
    #[derive(Clone)]
    pub struct Elem<'r, T> {
        next: List<'r, T>,
        value: T,
    }

    pub struct H2List;
    pub struct H1List<'a>(PhantomData<&'a ()>);
    impl<'a> PlugLifetime<'a> for H2List {
        type T = H1List<'a>;
    }
    impl<'a, T: 'a> PlugType<T> for H1List<'a> {
        type T = List<'a, <HTry<T> as PlugLifetime<'a>>::T>;
    }
    impl<'a, T> PlugLifetime<'a> for List<'a, T> {
        type T = List<'a, <HTry<T> as PlugLifetime<'a>>::T>;
    }
    impl<'b, A, B: 'b> PlugType<B> for List<'b, A> {
        type T = List<'b, B>;
    }

    pub struct H2Elem;
    pub struct H1Elem<'a>(PhantomData<&'a ()>);
    impl<'a> PlugLifetime<'a> for H2Elem {
        type T = H1Elem<'a>;
    }
    impl<'a, T: 'a> PlugType<T> for H1Elem<'a> {
        type T = Elem<'a, <HTry<T> as PlugLifetime<'a>>::T>;
    }
    impl<'a, T> PlugLifetime<'a> for Elem<'a, T> {
        type T = Elem<'a, <HTry<T> as PlugLifetime<'a>>::T>;
    }
    impl<'b, A, B: 'b> PlugType<B> for Elem<'b, A> {
        type T = Elem<'b, B>;
    }

    impl<'r, T> Elem<'r, T> {
        pub fn gc<'a: 'r>(
            arena: Arena<Static<T>>,
            next: impl TyEq<List<'r, T>>,
            value: impl TyEq<T>,
        ) -> Elem<'r, Ty<'r, T>> {
            let e = todo!();
        }
    }

    impl<'r, T: Copy> Copy for List<'r, T> {}
    impl<'r, T: Copy> Copy for Elem<'r, T> {}

    impl<'r, T> From<Gc<'r, Elem<'r, T>>> for List<'r, T> {
        fn from(e: Gc<'r, Elem<'r, T>>) -> Self {
            List(Some(e))
        }
    }

    impl<'o, T: Clone> List<'o, T> {
        /// Prepend `value` to a list.
        /// The arguments are in reverse order.
        pub fn cons<'r, 'a: 'r>(
            self,
            value: T,
            arena: &'a Arena<Static<T>>,
        ) -> List<'r, Ty<'r, T>> {
            let e: Elem<Ty<T>> = Elem::<T>::gc(todo!(), self, value);
            // let _ = List::from(e);
        }
    }
}
