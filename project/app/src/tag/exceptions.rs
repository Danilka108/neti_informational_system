use crate::SerialId;

#[derive(Debug, thiserror::Error)]
pub enum CreateTagException {
    #[error("tag name '{name}' is already in use")]
    NameIsAlreadyInUse { name: String },
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateTagNameException {
    #[error("tag name '{name}' is already in use")]
    NameIsAlreadyInUse { name: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteTagException {
    #[error("tag {id} does not exist")]
    TagDoesNotExist { id: SerialId },
}
