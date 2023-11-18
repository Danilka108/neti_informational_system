use std::num::NonZeroI32;

use crate::{
    person::Person,
    tag::Tag,
    university::{University, UniversityService},
    Outcome,
};

use super::{
    exceptions::*, BoxedSubdivisionMemberRepository, BoxedSubdivisionRepository,
    BoxedSubdivisionTagRepository, Subdivision, SubdivisionId, SubdivisionMember, SubdivisionTag,
};

pub struct SubdivisionService {
    pub(crate) repo: BoxedSubdivisionRepository,
    pub(crate) member_repo: BoxedSubdivisionMemberRepository,
    pub(crate) tag_repo: BoxedSubdivisionTagRepository,
    pub(crate) university_service: UniversityService,
}

impl SubdivisionService {
    pub async fn create(
        self,
        university: University,
        name: String,
    ) -> Outcome<Subdivision, CreateSubdivisionException> {
        let subdivision = Subdivision {
            id: SubdivisionId {
                university: university.clone(),
                name: name.clone(),
            },
        };

        let Ok(subdivision) = self.repo.insert(subdivision).await? else {
            return Outcome::Exception(CreateSubdivisionException::NameIsAlreadyInUse(
                NameIsAlreadyInUseError {
                    university_name: university.name,
                    subdivision_name: name,
                },
            ));
        };
        Outcome::Success(subdivision)
    }

    pub async fn delete(
        self,
        subdivision: Subdivision,
    ) -> Outcome<Subdivision, DeleteSubdivisionException> {
        let Ok(subdivision) = self.repo.delete(subdivision.id.clone()).await? else {
            return Outcome::Exception(DeleteSubdivisionException::DoesNotExist(
                SubdivisionDoesNotExistError {
                    name: subdivision.id.name,
                },
            ));
        };

        Outcome::Success(subdivision)
    }

    pub async fn update_name(
        self,
        id: SubdivisionId,
        name: String,
    ) -> Outcome<Subdivision, UpdateSubdivisionNameException> {
        let mut subdivision = self.get(id).await?;
        subdivision.id.name = name;

        let Ok(subdivision) = self.repo.update_name(subdivision.clone()).await? else {
            return Outcome::Exception(UpdateSubdivisionNameException::NameIsAlreadyInUse(
                NameIsAlreadyInUseError {
                    university_name: subdivision.id.university.name,
                    subdivision_name: subdivision.id.name,
                },
            ));
        };

        Outcome::Success(subdivision)
    }

    pub async fn get_tags(
        self,
        id: SubdivisionId,
    ) -> Outcome<Vec<Tag>, GetSubdivisionTagsException> {
        let subdivision = self.get(id).await?;
        let tags = self
            .tag_repo
            .get_by_subdivison(subdivision)
            .await?
            .into_iter()
            .map(|SubdivisionTag(_, tag)| tag)
            .collect();

        Outcome::Success(tags)
    }

    pub async fn add_tag(
        self,
        id: SubdivisionId,
        tag: Tag,
    ) -> Outcome<SubdivisionTag, AddSubdivisionTagException> {
        let subdivision = self.get(id).await?;

        let Ok(tag) = self
            .tag_repo
            .insert(SubdivisionTag(subdivision.clone(), tag.clone()))
            .await?
        else {
            return Outcome::Exception(AddSubdivisionTagException::TagIsAlreadyExist {
                subdivision_name: subdivision.id.name,
                tag_name: tag.name,
            });
        };

        Outcome::Success(tag)
    }

    pub async fn delete_tag(
        self,
        id: SubdivisionId,
        tag: Tag,
    ) -> Outcome<SubdivisionTag, DeleteSubdivisionTagException> {
        let subdivision = self.get(id).await?;

        let Ok(tag) = self
            .tag_repo
            .remove(SubdivisionTag(subdivision.clone(), tag.clone()))
            .await?
        else {
            return Outcome::Exception(DeleteSubdivisionTagException::TagDoesNotExist {
                subdivision_name: subdivision.id.name,
                tag_name: tag.name,
            });
        };

        Outcome::Success(tag)
    }

    pub async fn get_members(
        self,
        id: SubdivisionId,
    ) -> Outcome<Vec<SubdivisionMember>, GetSubdivisionMembersException> {
        let subdivision = self.get(id).await?;
        let members = self.member_repo.get_by_subdivison(subdivision).await?;

        Outcome::Success(members)
    }

    pub async fn add_member(
        self,
        id: SubdivisionId,
        person: Person,
        role: String,
    ) -> Outcome<SubdivisionMember, AddSubdivisionMemberException> {
        let subdivision = self.get(id).await?;

        let Ok(member) = self
            .member_repo
            .insert(SubdivisionMember {
                id: (subdivision.clone(), person.clone()),
                role,
            })
            .await?
        else {
            return Outcome::Exception(AddSubdivisionMemberException::MemberIsAlreadyExist {
                subdivision_name: subdivision.id.name,
                person_id: person.id,
            });
        };

        Outcome::Success(member)
    }

    pub async fn delete_member(
        self,
        id: SubdivisionId,
        person: Person,
    ) -> Outcome<SubdivisionMember, DeleteSubdivisionMemberException> {
        let subdivision = self.get(id).await?;

        let Ok(member) = self
            .member_repo
            .remove((subdivision.clone(), person.clone()))
            .await?
        else {
            return Outcome::Exception(DeleteSubdivisionMemberException::MemberDoesNotExist {
                person_id: person.id,
                subdivision_name: subdivision.id.name,
            });
        };

        Outcome::Success(member)
    }

    async fn get(&self, id: SubdivisionId) -> Outcome<Subdivision, SubdivisionDoesNotExistError> {
        let Ok(subdivision) = self.repo.get(id.clone()).await else {
            return Outcome::Exception(SubdivisionDoesNotExistError { name: id.name });
        };

        Outcome::Success(subdivision)
    }

    pub async fn get_by_university(
        self,
        university_id: NonZeroI32,
    ) -> Outcome<Vec<Subdivision>, GetSubdivisionsByUniversityException> {
        let university = self.university_service.get(university_id).await?;
        let subdivisions = self.repo.get_by_university(university).await?;

        Outcome::Success(subdivisions)
    }
}
