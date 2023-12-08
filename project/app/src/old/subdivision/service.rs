use crate::{
    person::{Person, PersonService},
    ports::{
        EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError, UniqualValueError,
    },
    tag::Tag,
    university::{University, UniversityService},
    Outcome, SerialId,
};

use super::{
    exceptions::*, BoxedSubdivisionMemberRepository, BoxedSubdivisionRepository,
    BoxedSubdivisionTagRepository, Subdivision, SubdivisionMember, SubdivisionTag,
};

pub struct SubdivisionService {
    pub(crate) repo: BoxedSubdivisionRepository,
    pub(crate) member_repo: BoxedSubdivisionMemberRepository,
    pub(crate) tag_repo: BoxedSubdivisionTagRepository,
    pub(crate) university_service: UniversityService,
    pub(crate) person_service: PersonService,
}

impl SubdivisionService {
    pub async fn create(
        self,
        university: University,
        name: String,
    ) -> Outcome<Subdivision, CreateSubdivisionException> {
        let subdivision = Subdivision {
            id: (),
            university: university.clone(),
            name: name.clone(),
        };

        self.repo
            .insert(subdivision)
            .await
            .map_exception(|EntityAlreadyExistError| {
                CreateSubdivisionException::NameIsAlreadyInUse(NameIsAlreadyInUseError {
                    university_name: university.name,
                    subdivision_name: name,
                })
            })
    }

    pub async fn delete(
        self,
        subdivision: Subdivision,
    ) -> Outcome<Subdivision, DeleteSubdivisionException> {
        self.repo
            .delete(subdivision.id.clone())
            .await
            .map_exception(|EntityDoesNotExistError| {
                DeleteSubdivisionException::DoesNotExist(SubdivisionDoesNotExistError {
                    id: subdivision.id,
                })
            })
    }

    pub async fn update_name(
        self,
        id: SerialId,
        name: String,
    ) -> Outcome<Subdivision, UpdateSubdivisionNameException> {
        let subdivision = self.get(id).await?;

        self.repo
            .update_name(id, name)
            .await
            .map_exception(|UniqualValueError| {
                UpdateSubdivisionNameException::NameIsAlreadyInUse(NameIsAlreadyInUseError {
                    university_name: subdivision.university.name,
                    subdivision_name: subdivision.name,
                })
            })
    }

    pub async fn get_tags(self, id: SerialId) -> Outcome<Vec<Tag>, GetSubdivisionTagsException> {
        let subdivision = self.get(id).await?;
        let tags = self
            .tag_repo
            .get_by_subdivison(subdivision)
            .await
            .collapse()?
            .into_iter()
            .map(|SubdivisionTag(_, tag)| tag)
            .collect();

        Outcome::Success(tags)
    }

    pub async fn add_tag(
        self,
        id: SerialId,
        tag: Tag,
    ) -> Outcome<SubdivisionTag, AddSubdivisionTagException> {
        let subdivision = self.get(id).await?;

        self.tag_repo
            .insert(SubdivisionTag(subdivision.clone(), tag.clone()))
            .await
            .map_exception(|EntityAlreadyExistError| {
                AddSubdivisionTagException::TagIsAlreadyExist {
                    subdivision_id: id,
                    tag_name: tag.name,
                }
            })
    }

    pub async fn delete_tag(
        self,
        id: SerialId,
        tag: Tag,
    ) -> Outcome<SubdivisionTag, DeleteSubdivisionTagException> {
        let subdivision = self.get(id).await?;

        self.tag_repo
            .remove(SubdivisionTag(subdivision.clone(), tag.clone()))
            .await
            .map_exception(|EntityDoesNotExistError| {
                DeleteSubdivisionTagException::TagDoesNotExist {
                    subdivision_id: id,
                    tag_name: tag.name,
                }
            })
    }

    pub async fn get_members(
        self,
        id: SerialId,
    ) -> Outcome<Vec<SubdivisionMember>, GetSubdivisionMembersException> {
        let subdivision = self.get(id).await?;
        let members = self
            .member_repo
            .get_by_subdivison(subdivision)
            .await
            .collapse()?;

        Outcome::Success(members)
    }

    pub async fn get_member(
        self,
        subdivision_id: SerialId,
        person_id: SerialId,
    ) -> Outcome<SubdivisionMember, GetSubdivisionMemberException> {
        let subdivision =
            self.repo
                .get(subdivision_id)
                .await
                .map_exception(|EntityNotFoundError| SubdivisionDoesNotExistError {
                    id: subdivision_id,
                })?;
        let person = self.person_service.get(person_id).await?;

        let member = self
            .member_repo
            .get((subdivision, person))
            .await
            .map_exception(|EntityDoesNotExistError| {
                GetSubdivisionMemberException::MemberDoesNotExist {
                    person_id,
                    subdivision_id,
                }
            })?;

        Outcome::Success(member)
    }

    pub async fn update_member_role(
        self,
        subdivision_id: SerialId,
        person_id: SerialId,
        role: String,
    ) -> Outcome<SubdivisionMember, UpdateSubdivisionMemberRoleException> {
        let subdivision =
            self.repo
                .get(subdivision_id)
                .await
                .map_exception(|EntityNotFoundError| SubdivisionDoesNotExistError {
                    id: subdivision_id,
                })?;
        let person = self.person_service.get(person_id).await?;

        let member = SubdivisionMember {
            id: (subdivision, person),
            role,
        };
        let updated_member = self.member_repo.update_role(member).await.map_exception(
            |EntityDoesNotExistError| UpdateSubdivisionMemberRoleException::MemberDoesNotExist {
                person_id,
                subdivision_id,
            },
        )?;

        Outcome::Success(updated_member)
    }

    pub async fn add_member(
        self,
        id: SerialId,
        person: Person,
        role: String,
    ) -> Outcome<SubdivisionMember, AddSubdivisionMemberException> {
        let subdivision = self.get(id).await?;

        self.member_repo
            .insert(SubdivisionMember {
                id: (subdivision.clone(), person.clone()),
                role,
            })
            .await
            .map_exception(|EntityAlreadyExistError| {
                AddSubdivisionMemberException::MemberIsAlreadyExist {
                    subdivision_id: id,
                    person_id: person.id,
                }
            })
    }

    pub async fn delete_member(
        self,
        id: SerialId,
        person: Person,
    ) -> Outcome<SubdivisionMember, DeleteSubdivisionMemberException> {
        let subdivision = self.get(id).await?;

        self.member_repo
            .remove((subdivision.clone(), person.clone()))
            .await
            .map_exception(|EntityDoesNotExistError| {
                DeleteSubdivisionMemberException::MemberDoesNotExist {
                    person_id: person.id,
                    subdivision_id: id,
                }
            })
    }

    pub async fn get_by_university(
        self,
        university_id: SerialId,
    ) -> Outcome<Vec<Subdivision>, GetSubdivisionsByUniversityException> {
        let university = self.university_service.get(university_id).await?;
        let subdivisions = self.repo.get_by_university(university).await.collapse()?;

        Outcome::Success(subdivisions)
    }

    async fn get(&self, id: SerialId) -> Outcome<Subdivision, SubdivisionDoesNotExistError> {
        self.repo
            .get(id)
            .await
            .map_exception(|EntityNotFoundError| SubdivisionDoesNotExistError { id })
    }
}
