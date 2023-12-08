use crate::{EntityTrait, FieldTrait, Id, Outcome};

pub type RepoOutcome<E, R = E> = Outcome<R, BaseRepoException<E>>;

#[async_trait::async_trait]
pub trait BaseRepo<E: EntityTrait> {
    async fn insert(&mut self, entity: E) -> RepoOutcome<E>;

    async fn update(&mut self, entity: E) -> RepoOutcome<E>;

    async fn find_by_id(&self, id: Id<E>) -> RepoOutcome<E>;

    async fn delete_by_id(&self, id: Id<E>) -> RepoOutcome<E>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub struct BaseRepoException<E> {
    msg: String,
    kind: BaseRepoKind,
    _marker: std::marker::PhantomData<E>,
}

#[derive(Debug, Clone, Copy)]
pub enum BaseRepoKind {
    NotFound,
    AlreadyExist,
    UniqueConstraintViolation,
    RefConstraintViolation,
}

impl<E> std::fmt::Display for BaseRepoException<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<E: EntityTrait> BaseRepoException<E> {
    fn new(msg: String, kind: BaseRepoKind) -> BaseRepoException<E> {
        Self {
            msg,
            kind,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn kind(&self) -> BaseRepoKind {
        self.kind
    }

    pub fn not_found<const FIELDS_COUNT: usize>(
        entity: E,
        targets: [E::Field; FIELDS_COUNT],
    ) -> BaseRepoException<E> {
        let msg = format!(
            "{} with fields {} not found",
            E::NAME,
            prettify_fields(entity, targets)
        );
        Self::new(msg, BaseRepoKind::NotFound)
    }

    pub fn alredy_exist<const FIELDS_COUNT: usize>(
        entity: E,
        targets: [E::Field; FIELDS_COUNT],
    ) -> BaseRepoException<E> {
        let msg = format!(
            "{} with fields {} already exist",
            E::NAME,
            prettify_fields(entity, targets)
        );
        Self::new(msg, BaseRepoKind::AlreadyExist)
    }

    pub fn unique_constraint_violation<const FIELDS_COUNT: usize>(
        unique_fields: [E::Field; FIELDS_COUNT],
    ) -> BaseRepoException<E> {
        let msg = format!(
            "fields {} of the {} must be unique together",
            prettify_fields_names::<FIELDS_COUNT, E>(unique_fields),
            E::NAME,
        );
        Self::new(msg, BaseRepoKind::UniqueConstraintViolation)
    }

    pub fn ref_constraint_violation<const FIELDS_COUNT: usize, T: EntityTrait>(
        from: [E::Field; FIELDS_COUNT],
        to: [T::Field; FIELDS_COUNT],
    ) -> BaseRepoException<E> {
        let msg = format!(
            "fields {} of the {} must be valid reference to fields {} of the {}",
            prettify_fields_names::<FIELDS_COUNT, E>(from),
            E::NAME,
            prettify_fields_names::<FIELDS_COUNT, T>(to),
            T::NAME,
        );
        Self::new(msg, BaseRepoKind::RefConstraintViolation)
    }
}

fn prettify_fields<const COUNT: usize, E: EntityTrait>(
    entity: E,
    fields: [E::Field; COUNT],
) -> String {
    prettify_fields_with::<COUNT, E>(fields, |f| {
        format!(r#""{}" = "{}""#, f.name(), entity.get_field_value(f))
    })
}

fn prettify_fields_names<const COUNT: usize, E: EntityTrait>(fields: [E::Field; COUNT]) -> String {
    prettify_fields_with::<COUNT, E>(fields, |f| format!(r#""{}""#, f.name()))
}

fn prettify_fields_with<const COUNT: usize, E: EntityTrait>(
    fields: [E::Field; COUNT],
    mapper: impl Fn(E::Field) -> String,
) -> String {
    let mut result = String::new();
    let mut fields = fields.into_iter().peekable();

    loop {
        let Some(field) = fields.next() else {
            break;
        };

        result.push_str(&mapper(field));

        if fields.peek().is_some() {
            result.push_str(", ");
        }
    }

    result
}
