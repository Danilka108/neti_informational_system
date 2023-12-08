DROP SCHEMA IF EXISTS public CASCADE;
CREATE SCHEMA public;

CREATE DOMAIN seconds_from_unix_epoch bigint CHECK (value > 0);

CREATE TYPE gender AS ENUM ('MALE', 'FEMALE');

CREATE FUNCTION is_numeric(text) RETURNS boolean AS
    'SELECT $1 ~ ''^[0-9]+$'' ' LANGUAGE 'sql';

CREATE TYPE user_role AS ENUM ('ADMIN');

CREATE TYPE teacher_kind AS enum ('assistant', 'regular_teacher', 'senior_teacher', 'associate_professor', 'professor');

CREATE TYPE qualification AS enum ('bachelor', 'master', 'postgraduate', 'doctorate');

CREATE TYPE training_kind AS enum ('FULL_TIME', 'CORRESPONDENCE');

CREATE TYPE attestation_kind AS enum ('TEST', 'DIFF_TEST', 'EXAM');

CREATE TABLE persons
(
    id serial
        PRIMARY KEY,
    email varchar(255) NOT NULL
        UNIQUE,
    hashed_password text NOT NULL
    -- role user_role NOT NULL,
);

CREATE TABLE person_sessions
(
    person_id serial NOT NULL references persons,
    metadata varchar(1024) NOT NULL,
    refresh_token varchar(1024) NOT NULL
        UNIQUE,
    expires_at_in_seconds integer NOT NULL,

    PRIMARY KEY (person_id, metadata)
);

-- CREATE TABLE users
-- (
--     id serial NOT NULL
--         PRIMARY KEY,
--     -- person_id serial not null references persons,
--     email varchar(255) NOT NULL
--         UNIQUE,
--     role user_role NOT NULL,
--     hashed_password text NOT NULL
-- );

-- CREATE TABLE user_sessions
-- (
--     user_id serial NOT NULL references users,
--     metadata varchar(1024) NOT NULL,
--     refresh_token varchar(1024) NOT NULL
--         UNIQUE,
--     expires_at_in_seconds integer NOT NULL,

--     PRIMARY KEY (user_id, metadata)
-- );

create table universities
(
  id serial primary key,
  name varchar(256) not null UNIQUE
);

create table subdivision_attributes
(
  id serial primary key,
  name varchar(128) unique not null
);

create table subdivisions
(
  id serial primary key,
  university_id serial not null references universities,
  name varchar(256) not null,

  unique (university_id, name)
);

create table subdivisions_attributes
(
  attribute_id serial not null references subdivision_attributes,
  subdivision_id serial not null references subdivisions,

  primary key (attribute_id, subdivision_id)
);

create table subdivision_members
(
  id serial primary key,
  subdivision_id serial NOT NULL REFERENCES subdivisions,
  person_id serial not null references persons,
  role varchar(512) not null,

  unique (subdivision_id, person_id)
);

create table study_groups
(
  id serial primary key,
  name varchar(256) not null unique,
  department_id serial not null references subdivisions,
  studying_qualification qualification not null,
  training_kind training_kind NOT NULL
);

create table students
(
  id serial primary key references persons
);

create table study_groups_students
(
  study_group_id serial not null references study_groups,
  student_id serial not null references students,

  primary key (study_group_id, student_id)
);

create table teachers
(
  id serial primary key references persons,
  kind teacher_kind NOT NULL,
  department_id serial NOT NULL references subdivisions
);

create table curriculums
(
  id serial primary key
);

create table study_groups_curriculums
(
  study_group_id serial not null references study_groups,
  curriculum_id serial not null references curriculums,

  primary key (study_group_id, curriculum_id)
);

create table discipline_types
(
  id serial primary key,
  department_id serial not null references subdivisions,
  name varchar(256) not null
);

create table disciplines
(
  id serial primary key,
  curriculum_id serial not null references curriculums,
  type_id serial not null references discipline_types,
  semester integer NOT NULL CHECK (semester > 0)
);

CREATE TABLE attestations
(
  id serial primary key,
  discipline_id serial not null references disciplines,
  kind attestation_kind NOT NULL,
  duration_in_hours float NOT NULL
      CHECK (duration_in_hours > 0)
);

CREATE TABLE attestations_examiners
(
    examiners_id serial NOT NULL references teachers,
    attestation_id serial NOT NULL references attestations,

    PRIMARY KEY (examiners_id, attestation_id)
);

CREATE TABLE students_attestations
(
    student_id serial NOT NULL references students,
    attestation_id serial NOT NULL references attestations,
    score integer NOT NULL
        CONSTRAINT students_attestations_score_check
            CHECK (score >= 0 AND score <= 100),
    rating_contributor_id serial references persons,

    PRIMARY KEY (student_id, attestation_id)
);

create table class_kinds
(
  id serial primary key,
  name varchar(256) not null
);

CREATE TABLE classes
(
    id serial
        PRIMARY KEY,
    discipline_id serial NOT NULL references disciplines,
    kind serial not null references class_kinds,
    duration_in_hours float NOT NULL
        CHECK (duration_in_hours > 0)
);

create table classes_teachers
(
  teacher_id serial not null references teachers,
  class_id serial not null references classes,
  study_group_id serial not null references study_groups,

  primary key (teacher_id, class_id, study_group_id)
);

-- DROP SCHEMA IF EXISTS public CASCADE;
-- CREATE SCHEMA public;

-- CREATE DOMAIN seconds_from_unix_epoch bigint CHECK (value > 0);

-- CREATE TYPE gender AS ENUM ('MALE', 'FEMALE');

-- CREATE FUNCTION is_numeric(text) RETURNS boolean AS
--     'SELECT $1 ~ ''^[0-9]+$'' ' LANGUAGE 'sql';

-- CREATE TYPE user_role AS ENUM ('ADMIN');

-- CREATE TYPE teacher_kind AS enum ('assistant', 'regular_teacher', 'senior_teacher', 'associate_professor', 'professor');

-- CREATE TYPE qualification AS enum ('bachelor', 'master', 'postgraduate', 'doctorate');

-- CREATE TYPE training_kind AS enum ('FULL_TIME', 'CORRESPONDENCE');

-- CREATE TYPE attestation_kind AS enum ('TEST', 'DIFF_TEST', 'EXAM');

-- CREATE TABLE users
-- (
--     id serial NOT NULL
--         PRIMARY KEY,
--     email varchar(255) NOT NULL
--         UNIQUE,
--     role user_role NOT NULL,
--     hashed_password text NOT NULL
-- );

-- CREATE TABLE user_sessions
-- (
--     user_id serial NOT NULL,
--     metadata varchar(1024) NOT NULL,
--     refresh_token varchar(1024) NOT NULL
--         UNIQUE,
--     expires_at_in_seconds integer NOT NULL,

--     PRIMARY KEY (user_id, metadata)
-- );


-- CREATE TABLE persons
-- (
--     id serial
--         PRIMARY KEY
-- );


-- create table universities
-- (
--   id serial primary key,
--   name varchar(256) not null UNIQUE
-- );

-- create table tags
-- (
--   id serial primary key,
--   name varchar(128) unique not null
-- );

-- create table subdivisions
-- (
--   id serial primary key,
--   university_id serial not null references universities,
--   name varchar(256) not null,

--   unique (university_id, name)
-- );

-- create table subdivisions_tags
-- (
--   tag_id serial not null references tags,
--   subdivision_id serial not null references subdivisions,

--   primary key (tag_id, subdivision_id)
-- );

-- create table subdivision_members
-- (
--   id serial primary key,
--   subdivision_id serial NOT NULL REFERENCES subdivisions,
--   person_id serial not null references persons,
--   role varchar(512) not null,

--   unique (subdivision_id, person_id)
-- );

-- create table study_groups
-- (
--   id serial primary key,
--   name varchar(256) not null unique,
--   department_id serial not null references subdivisions,
--   studying_qualification qualification not null,
--   training_kind training_kind NOT NULL
-- );

-- create table students
-- (
--   id serial primary key references persons
-- );

-- create table study_groups_students
-- (
--   study_group_id serial not null references study_groups,
--   student_id serial not null references students,

--   primary key (study_group_id, student_id)
-- );

-- create table teachers
-- (
--   id serial primary key references persons,
--   kind teacher_kind NOT NULL,
--   department_id serial NOT NULL references subdivisions
-- );

-- create table curriculums
-- (
--   id serial primary key
-- );

-- create table study_groups_curriculums
-- (
--   study_group_id serial not null references study_groups,
--   curriculum_id serial not null references curriculums,

--   primary key (study_group_id, curriculum_id)
-- );

-- create table disciplines
-- (
--   id serial primary key,
--   name varchar(256) not null,
--   department_id serial not null references subdivisions
-- );

-- create table curriculum_items
-- (
--   id serial primary key,
--   curriculum_id serial not null references curriculums,
--   discipline_id serial not null references disciplines,
--   semester integer NOT NULL CHECK (semester > 0)
-- );

-- CREATE TABLE attestations
-- (
--   id serial primary key,
--   curriculum_item_id serial not null references curriculum_items,
--   kind attestation_kind NOT NULL,
--   duration_in_hours float NOT NULL
--       CHECK (duration_in_hours > 0)
-- );

-- CREATE TABLE attestations_examiners
-- (
--     examiners_id serial NOT NULL references teachers,
--     attestation_id serial NOT NULL references attestations,

--     PRIMARY KEY (examiners_id, attestation_id)
-- );

-- CREATE TABLE students_attestations
-- (
--     student_id serial NOT NULL references students,
--     attestation_id serial NOT NULL references attestations,
--     score integer NOT NULL
--         CONSTRAINT students_attestations_score_check
--             CHECK (score >= 0 AND score <= 100),
--     rating_contributor_id serial references persons,

--     PRIMARY KEY (student_id, attestation_id)
-- );

-- create table class_kinds
-- (
--   id serial primary key,
--   name varchar(256) not null
-- );

-- CREATE TABLE classes
-- (
--     id serial
--         PRIMARY KEY,
--     curriculum_item_id serial NOT NULL references curriculum_items,
--     kind serial not null references class_kinds,
--     duration_in_hours float NOT NULL
--         CHECK (duration_in_hours > 0)
-- );

-- create table classes_teachers
-- (
--   teacher_id serial not null references teachers,
--   class_id serial not null references classes,
--   study_group_id serial not null references study_groups,

--   primary key (teacher_id, class_id, study_group_id)
-- );
