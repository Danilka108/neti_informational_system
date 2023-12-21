use std::hash::Hash;

#[derive(Debug, Clone, Copy, Default)]
pub struct LazyAttr;

pub trait AttrTrait: PartialEq + Eq {
    // fn name(&self) -> &'static str;
}
