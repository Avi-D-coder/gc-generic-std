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

pub struct H2Gc;
pub struct H1Gc<'a>(PhantomData<&'a ()>);
impl<'a> PlugLifetime<'a> for H2Gc {
    type T = H1Gc<'a>;
}
impl<'a, T: 'a + PlugLifetime<'a>> PlugType<T> for H1Gc<'a> {
    type T = Gc<'a, <T as PlugLifetime<'a>>::T>;
}
impl<'a, T: 'static> PlugType<T> for H1Gc<'a> {
    default type T = Gc<'a, T>;
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
impl<'a, T: 'static> PlugType<T> for H1List<'a> {
    default type T = List<'a, T>;
}



pub struct H2Elem;
pub struct H1Elem<'a>(PhantomData<&'a ()>);
impl<'a> PlugLifetime<'a> for H2Elem {
    type T = H1Elem<'a>;
}
impl<'a, T: 'a + PlugLifetime<'a>> PlugType<T> for H1Elem<'a> {
    type T = Elem<'a, <T as PlugLifetime<'a>>::T>;
}
impl<'a, T: 'static> PlugType<T> for H1Elem<'a> {
    default type T = Elem<'a, T>;
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

pub struct HTry<T>(PhantomData<T>);
impl<'a, T> PlugLifetime<'a> for HTry<T> {
    default type T = T;
}
impl<'a, T: PlugLifetime<'a>> PlugLifetime<'a> for HTry<T> {
    type T = <T as PlugLifetime<'a>>::T;
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
