mod attr;
mod id;
pub mod method;

pub use attr::{AttrTrait, LazyAttr};
pub use id::{Id, ProvideId};
pub use utils_macros::entity;
pub use utils_macros::entity_method;

pub trait EntityTrait: Sized {
    // const NAME: &'static str;

    type Attr: AttrTrait;
    type IdValue;

    // fn id_attr() -> Self::Attr;
    // fn non_id_attrs() -> Vec<Self::Attr>;
}
