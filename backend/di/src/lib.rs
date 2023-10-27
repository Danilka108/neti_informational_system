#![feature(trait_alias)]

extern crate di_macro;

// pub use di_macro::FromModule;

pub trait Provide<T: Sized> {
    fn provide(&self) -> T;
}

pub trait Module {
    fn resolve<C>(&self) -> C
    where
        Self: Provide<C>,
    {
        self.provide()
    }
}
