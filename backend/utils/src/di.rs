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
