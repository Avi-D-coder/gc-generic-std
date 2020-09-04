#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(marker_trait_attr)]
#![feature(negative_impls)]
#![feature(optin_builtin_traits)]

use auto_traits::NoGc;
use generic_std::plug::*;
use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}

pub trait UnPlugType {
    type T;
}

pub trait UnPlugLifetime {
    type T;
}

/// Realy `Gc<'r, T>(&'r T<'r>);`
pub struct Gc<'r, T>(&'r T);

pub struct GcLifeless<T>(PhantomData<T>);
impl<'r, T> UnPlugLifetime for Gc<'r, T> {
    type T = GcLifeless<T>;
}
impl<'r, T: UnPlugLifetime> PlugLifetime<'r> for GcLifeless<T> where <T as UnPlugLifetime>::T: PlugLifetime<'r>, <<T as UnPlugLifetime>::T as PlugLifetime<'r>>::T: 'r {
    type T = Gc<'r, <<T as UnPlugLifetime>::T as PlugLifetime<'r>>::T>;
}


// fn transmute_lifetime<'a, 'b, T: UnPlugType>(
//     from: T,
// ) -> <<<<T as UnPlugType>::T as UnPlugLifetime>::T as PlugLifetime<'b>>::T as PlugType<T>>::T
// where
//     <T as UnPlugType>::T: UnPlugLifetime,
//     <<T as UnPlugType>::T as UnPlugLifetime>::T: generic_std::plug::PlugLifetime<'b>,
//     <<<T as UnPlugType>::T as UnPlugLifetime>::T as generic_std::plug::PlugLifetime<'b>>::T:
//         generic_std::plug::PlugType<T>,
// {
//     todo!()
// }

#[test]
fn unify_test() {
    fn foo<A, B: Id<A>>() {}
    foo::<usize, usize>();
    foo::<Gc<usize>, Gc<usize>>();

    fn lifes<'a, 'b, T: for<'l> PlugLifetime<'l>>() {
        // let a: Gc<'a, usize> = Gc(&1);
        // let b: Gc<'b, usize> = transmute_lifetime(a);
        // foo::<<<T as PlugLifetime<'a>>::T as PlugLifetime<'b>>::T, <T as PlugLifetime<'b>>::T>();
        // foo::<Gc<'a, usize>, Gc<'a, Ty<'a, usize>>>();
    }
    // foo::<Gc<usize>, Gc<Ty<Ty<String>>>>();
}

pub struct List<'r, T>(Option<Gc<'r, Elem<'r, T>>>);
pub struct Elem<'r, T> {
    next: List<'r, T>,
    value: T,
}

pub struct H2List;
pub struct H1List<'a>(PhantomData<&'a ()>);
impl<'a> PlugLifetime<'a> for H2List {
    type T = H1List<'a>;
}
impl<'a, T: 'a + PlugLifetime<'a>> PlugType<T> for H1List<'a> {
    type T = List<'a, <T as PlugLifetime<'a>>::T>;
}

pub struct H2Elem;
pub struct H1Elem<'a>(PhantomData<&'a ()>);
impl<'a> PlugLifetime<'a> for H2Elem {
    type T = H1Elem<'a>;
}
impl<'a, T: 'a + PlugLifetime<'a>> PlugType<T> for H1Elem<'a> {
    type T = Elem<'a, <T as PlugLifetime<'a>>::T>;
}

impl<'r, T> Elem<'r, T> {
    pub fn gc<'a: 'r, A: Static>(
        arena: Arena<<A as Static>::LifeTime>,
        next: impl TyEq<List<'r, T>>,
        value: impl TyEq<T>,
    ) -> Self {
        todo!()
    }
}

pub unsafe trait Id<T> {}
unsafe impl<T> Id<T> for T {}

#[marker]
pub unsafe trait TyEq<B> {}
// unsafe impl<T> TyEq<T> for T {}
unsafe impl<'a, A: PlugLifetime<'a>, B: PlugLifetime<'a>> TyEq<A> for B where A::T: Id<B::T> {}
unsafe impl<'a, A: Static, B: Static> TyEq<A> for B where A::LifeTime: Id<B::LifeTime> {}

pub trait Static {
    type LifeTime: 'static;
}

impl<T: 'static> Static for T {
    type LifeTime = T;
}

pub trait Trace {
    type Arena;
}

pub struct Arena<A>(Vec<A>);

mod auto_traits {
    use super::*;
    use std::cell::UnsafeCell;

    pub unsafe auto trait NoGc {}
    impl<'r, T> !NoGc for Gc<'r, T> {}
    // unsafe impl<'r, T: NoGc> NoGc for Box<T> {}

    pub trait HasGc {
        const HAS_GC: bool;
    }

    impl<T> HasGc for T {
        default const HAS_GC: bool = true;
    }

    impl<T: NoGc> HasGc for T {
        const HAS_GC: bool = false;
    }

    /// Shallow immutability
    pub unsafe auto trait Immutable {}
    impl<T> !Immutable for &mut T {}
    impl<'r, T> !Immutable for &'r T {}
    impl<T> !Immutable for UnsafeCell<T> {}
    unsafe impl<T> Immutable for Box<T> {}
    unsafe impl<'r, T> Immutable for Gc<'r, T> {}

    /// Should be implemented with each `Trace` impl.
    pub auto trait NotDerived {}
    impl<'l, T> !NotDerived for Gc<'l, T> {}
}
