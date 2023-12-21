use utils::{
    entity::{AttrTrait, EntityTrait, Id},
    outcome::Outcome,
};

use crate::curriculum;

type EntityId = Id<Entity>;

#[derive(Debug, Clone)]
pub struct Entity {
    pub study_group_id: super::EntityId,
    pub curriculum_id: curriculum::EntityId,
}

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn find(
        &mut self,
        study_group_id: super::EntityId,
        curriculum_id: curriculum::EntityId,
    ) -> Result<Entity, anyhow::Error>;

    async fn delete(
        &mut self,
        study_group_id: super::EntityId,
        curriculum_id: curriculum::EntityId,
    ) -> Result<Entity, anyhow::Error>;

    async fn 
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum EntityAttr {
//     StudyGroupId,
//     CurriculumId,
// }

// impl AttrTrait for EntityAttr {
//     fn name(&self) -> &'static str {
//         unimplemented!()
//     }
// }

// impl EntityTrait for Entity {
//     const NAME: &'static str = "study_group_curriculums";
//     type Attr = EntityAttr;
//     type IdValue = (super::EntityId, curriculum::EntityId);

//     fn id_attr() -> Self::Attr {
//         unimplemented!()
//     }

//     fn non_id_attrs() -> Vec<Self::Attr> {
//         unimplemented!()
//     }
// }
