mod id;
pub mod method;

pub use id::{Id, ProvideId};
pub use utils_macros::entity;
pub use utils_macros::entity_method;

pub trait EntityTrait {
    const NAME: &'static str;

    type Attr: AttrTrait;
    type IdValue;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LazyAttr;

pub trait AttrTrait: PartialEq {
    fn name(&self) -> &'static str;
}
