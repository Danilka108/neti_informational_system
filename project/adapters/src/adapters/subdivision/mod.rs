use std::{convert::Infallible, num::NonZeroI32, sync::Arc};
use tokio::sync::Mutex;

use super::ProvideTxn;
use crate::{
    adapters::subdivision::models::{PgSubdivisionMember, PgSubdivisionTag},
    pg::PgTransaction,
};
use app::{
    person::Person,
    ports::{
        EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError, UniqualValueError,
    },
    subdivision::{
        Subdivision, SubdivisionMember, SubdivisionMemberRepository, SubdivisionRepository,
        SubdivisionTag, SubdivisionTagRepository,
    },
    university::University,
    Outcome,
};

mod models;

use models::PgSubdivision;

pub struct PgSubdivisionRepository {
    pub txn: Arc<Mutex<PgTransaction>>,
}

pub struct PgSubdivisionMemberRepository {
    pub txn: Arc<Mutex<PgTransaction>>,
}

pub struct PgSubdivisionTagRepository {
    pub txn: Arc<Mutex<PgTransaction>>,
}

impl ProvideTxn for PgSubdivisionRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

impl ProvideTxn for PgSubdivisionMemberRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

impl ProvideTxn for PgSubdivisionTagRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

#[async_trait::async_trait]
impl SubdivisionRepository for PgSubdivisionRepository {
    async fn insert(
        &self,
        subdivision: Subdivision<()>,
    ) -> Outcome<Subdivision, EntityAlreadyExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivision,
            r#"
                WITH add_subdivision AS (
                    INSERT
                        INTO subdivisions (name, university_id)
                        VALUES ($1, $2)
                        ON CONFLICT DO NOTHING
                        RETURNING id, name, university_id
                )
                SELECT a.id AS subdivision_id, a.name AS subdivision_name, u.id AS university_id, u.name AS university_name
                    FROM add_subdivision AS a INNER JOIN universities AS u ON u.id = a.university_id;
            "#,
            &subdivision.name,
            subdivision.university.id.get()
        ))
        .await
    }

    async fn delete(&self, id: NonZeroI32) -> Outcome<Subdivision, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivision,
            r#"
                WITH delete_subdivision AS (
                    DELETE
                        FROM subdivisions
                        WHERE id = $1
                        RETURNING id, name, university_id
                )
                SELECT d.id AS subdivision_id, d.name AS subdivision_name, u.id as university_id, u.name AS university_name
                    FROM delete_subdivision AS d INNER JOIN universities as u ON d.university_id = u.id;
            "#,
            id.get(),
        ))
        .await
    }

    async fn update_name(
        &self,
        id: NonZeroI32,
        name: String,
    ) -> Outcome<Subdivision, UniqualValueError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivision,
            r#"
                WITH update_subdivision AS (
                    UPDATE
                        subdivisions
                        SET
                            name = $1
                        WHERE
                            id = $2
                        RETURNING id, name, university_id
                )
                SELECT s.id AS subdivision_id, s.name AS subdivision_name, u.id as university_id, u.name AS university_name
                    FROM update_subdivision AS s
                        INNER JOIN universities as u ON s.university_id = u.id;
            "#,
            &name,
            id.get(),
        ))
        .await
    }

    async fn get(&self, id: NonZeroI32) -> Outcome<Subdivision, EntityNotFoundError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivision,
            r#"
                SELECT s.id AS subdivision_id, s.name AS subdivision_name, u.id AS university_id, u.name AS university_name
                    FROM subdivisions AS s INNER JOIN universities AS u ON s.university_id = u.id
                    WHERE s.id = $1;
            "#,
            id.get(),
        ))
        .await
    }

    async fn get_by_university(
        &self,
        university: University,
    ) -> Outcome<Vec<Subdivision>, Infallible> {
        self.fetch_all(sqlx::query_as!(
            PgSubdivision,
            r#"
                SELECT s.id AS subdivision_id, s.name AS subdivision_name, u.id AS university_id, u.name AS university_name
                    FROM subdivisions AS s
                        INNER JOIN universities AS u ON s.university_id = u.id
                    WHERE u.id = $1;
            "#,
            university.id.get(),
        ))
        .await
    }
}

#[async_trait::async_trait]
impl SubdivisionMemberRepository for PgSubdivisionMemberRepository {
    async fn insert(
        &self,
        member: SubdivisionMember,
    ) -> Outcome<SubdivisionMember, EntityAlreadyExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivisionMember,
            "
                WITH insert_member AS (
                    INSERT
                        INTO subdivision_members (subdivision_id, person_id, role)
                        VALUES ($1, $2, $3)
                        ON CONFLICT DO NOTHING
                        RETURNING *
                )
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    p.id AS person_id,
                    i.role AS role
                FROM insert_member AS i
                    INNER JOIN subdivisions AS s ON s.id = i.subdivision_id
                    INNER JOIN persons AS p ON p.id = i.person_id
                    INNER JOIN universities as u ON u.id = s.university_id;
            ",
            member.id.0.id.get(),
            member.id.1.id.get(),
            &member.role,
        ))
        .await
    }

    async fn remove(
        &self,
        id: (Subdivision, Person),
    ) -> Outcome<SubdivisionMember, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivisionMember,
            "
                WITH delete_member AS (
                    DELETE
                        FROM subdivision_members
                        WHERE subdivision_id = $1 AND person_id = $2
                        RETURNING *
                )
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    p.id AS person_id,
                    d.role AS role
                FROM delete_member AS d
                    INNER JOIN subdivisions AS s ON s.id = d.subdivision_id
                    INNER JOIN persons AS p ON p.id = d.person_id
                    INNER JOIN universities as u ON u.id = s.university_id;
            ",
            id.0.id.get(),
            id.1.id.get(),
        ))
        .await
    }

    async fn update_role(
        &self,
        member: SubdivisionMember,
    ) -> Outcome<SubdivisionMember, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivisionMember,
            "
                WITH update_member_role AS (
                    UPDATE subdivision_members
                    SET role = $3
                    WHERE subdivision_id = $1 AND person_id = $2
                    RETURNING *
                )
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    p.id AS person_id,
                    um.role AS role
                FROM update_member_role AS um
                    INNER JOIN subdivisions AS s ON s.id = um.subdivision_id
                    INNER JOIN persons AS p ON p.id = um.person_id
                    INNER JOIN universities as u ON u.id = s.university_id;
            ",
            member.id.0.id.get(),
            member.id.1.id.get(),
            &member.role,
        ))
        .await
    }

    async fn get(
        &self,
        id: (Subdivision, Person),
    ) -> Outcome<SubdivisionMember, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivisionMember,
            "
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    p.id AS person_id,
                    sm.role AS role
                FROM subdivision_members AS sm
                    INNER JOIN subdivisions AS s ON s.id = sm.subdivision_id
                    INNER JOIN persons AS p ON p.id = sm.person_id
                    INNER JOIN universities as u ON u.id = s.university_id
                WHERE sm.subdivision_id = $1 AND sm.person_id = $2;
            ",
            id.0.id.get(),
            id.1.id.get(),
        ))
        .await
    }

    async fn get_by_subdivison(
        &self,
        subdivision: Subdivision,
    ) -> Outcome<Vec<SubdivisionMember>, Infallible> {
        self.fetch_all(sqlx::query_as!(
            PgSubdivisionMember,
            "
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    p.id AS person_id,
                    sm.role AS role
                FROM subdivision_members AS sm
                    INNER JOIN subdivisions AS s ON s.id = sm.subdivision_id
                    INNER JOIN persons AS p ON p.id = sm.person_id
                    INNER JOIN universities as u ON u.id = s.university_id
                WHERE sm.subdivision_id = $1;
            ",
            subdivision.id.get(),
        ))
        .await
    }
}

#[async_trait::async_trait]
impl SubdivisionTagRepository for PgSubdivisionTagRepository {
    async fn insert(
        &self,
        tag: SubdivisionTag,
    ) -> Outcome<SubdivisionTag, EntityAlreadyExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivisionTag,
            "
                WITH insert_subdivision_tag AS (
                    INSERT
                        INTO subdivisions_tags (subdivision_id, tag_id)
                        VALUES ($1, $2)
                        ON CONFLICT DO NOTHING
                        RETURNING *
                )
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    t.id AS tag_id, t.name AS tag_name
                FROM insert_subdivision_tag AS a
                    INNER JOIN subdivisions AS s ON s.id = a.subdivision_id
                    INNER JOIN tags AS t ON t.id = a.tag_id
                    INNER JOIN universities AS u ON u.id = s.university_id;
            ",
            tag.0.id.get(),
            tag.1.id.get(),
        ))
        .await
    }

    async fn remove(
        &self,
        tag: SubdivisionTag,
    ) -> Outcome<SubdivisionTag, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSubdivisionTag,
            "
                WITH delete_subdivision_tag AS (
                    DELETE
                        FROM subdivisions_tags
                        WHERE subdivision_id = $1 AND tag_id = $2
                        RETURNING *
                )
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    t.id AS tag_id, t.name AS tag_name
                FROM delete_subdivision_tag AS a
                    INNER JOIN subdivisions AS s ON s.id = a.subdivision_id
                    INNER JOIN tags AS t ON t.id = a.tag_id
                    INNER JOIN universities AS u ON u.id = s.university_id;
            ",
            tag.0.id.get(),
            tag.1.id.get(),
        ))
        .await
    }

    async fn get_by_subdivison(
        &self,
        subdivision: Subdivision,
    ) -> Outcome<Vec<SubdivisionTag>, Infallible> {
        self.fetch_all(sqlx::query_as!(
            PgSubdivisionTag,
            "
                SELECT
                    s.id AS subdivision_id, s.name AS subdivision_name,
                    u.id AS university_id, u.name AS university_name,
                    t.id AS tag_id, t.name AS tag_name
                FROM subdivisions_tags AS st
                    INNER JOIN subdivisions AS s ON s.id = st.subdivision_id
                    INNER JOIN tags AS t ON t.id = st.tag_id
                    INNER JOIN universities AS u ON u.id = s.university_id
                WHERE st.subdivision_id = $1;
            ",
            subdivision.id.get(),
        ))
        .await
    }
}
