DROP SCHEMA IF EXISTS public CASCADE;
CREATE SCHEMA public;

CREATE DOMAIN seconds_from_unix_epoch bigint CHECK (value > 0);

CREATE TYPE gender AS ENUM ('male', 'female');

CREATE FUNCTION is_numeric(text) RETURNS boolean AS
    'SELECT $1 ~ ''^[0-9]+$'' ' LANGUAGE 'sql';

-- CREATE TYPE user_role AS ENUM ('ADMIN');

CREATE TYPE teacher_kind AS enum ('assistant', 'regular_teacher', 'senior_teacher', 'associate_professor', 'professor');

CREATE TYPE qualification AS enum ('bachelor', 'master', 'postgraduate', 'doctorate');

CREATE TYPE training_kind AS enum ('full_time', 'correspondence');

CREATE TYPE attestation_kind AS enum ('test', 'diff_test', 'exam');


create table users (
  id serial primary key,
  email varchar(256) not null unique,
  password text not null
);

CREATE TABLE user_sessions
(
    user_id serial NOT NULL references users,
    metadata varchar(1024) NOT NULL,
    refresh_token varchar(1024) NOT NULL
        UNIQUE,
    expires_at seconds_from_unix_epoch NOT NULL,

    PRIMARY KEY (user_id, metadata)
);

CREATE TABLE persons
(
    id serial
        PRIMARY KEY,
    user_id serial unique not null references users,
    full_name varchar(1024) not null
);

create table passports(
  id serial primary key,
  person_id serial not null REFERENCES persons,
  first_name varchar(256) not null,
  last_name varchar(256) not null,
  patronymic varchar(256) not null,
  date_of_birth timestamp not null,
  date_of_issue timestamp not null,
  number varchar(6) not null,
  series varchar(4) not null,
  gender gender not null
);

create table universities
(
  id serial primary key,
  name varchar(256) not null UNIQUE
);

create table tags
(
  name varchar(128) primary key
);

create table subdivisions
(
  id serial primary key,
  university_id serial not null references universities,
  name varchar(256) not null,

  unique (university_id, name)
);

create table subdivision_tags
(
  tag_name varchar(128) not null references tags,
  subdivision_id serial not null references subdivisions,

  primary key (tag_name, subdivision_id)
);

create table subdivision_members
(
  -- id serial primary key,
  person_id serial not null references persons,
  subdivision_id serial NOT NULL REFERENCES subdivisions,
  role varchar(512) not null,

  primary key (subdivision_id, person_id)
);

create table study_groups
(
  id serial primary key,
  name varchar(256) not null unique,
  department_id serial not null references subdivisions,
  studying_qualification qualification not null,
  training_kind training_kind NOT NULL
);

create table teachers
(
  id serial primary key,
  person_id serial not null unique references persons,
  kind teacher_kind NOT NULL,
  department_id serial NOT NULL references subdivisions
);

create table students
(
  id serial primary key,
  person_id serial not null references persons,
  study_group_id serial not null references study_groups,

  unique (person_id, study_group_id)
);

-- create table study_groups_students
-- (
--   study_group_id serial not null references study_groups,
--   student_id serial not null references students,

--   primary key (study_group_id, student_id)
-- );

create table curriculums
(
  id serial primary key,
  name varchar(256) not null unique
);

create table study_group_curriculums
(
  study_group_id serial not null references study_groups,
  curriculum_id serial not null references curriculums,

  primary key (study_group_id, curriculum_id)
);

create table disciplines
(
  id serial primary key,
  department_id serial not null references subdivisions,
  name varchar(256) not null
);

create table curriculum_modules
(
  id serial primary key,
  curriculum_id serial not null references curriculums,
  discipline_id serial not null references disciplines,
  semester integer NOT NULL CHECK (semester > 0)
);

CREATE TABLE attestations
(
  id serial primary key,
  curriculum_module_id serial not null unique references curriculum_modules,
  kind attestation_kind NOT NULL,
  duration_in_hours float NOT NULL
      CHECK (duration_in_hours > 0)
);

CREATE TABLE attestation_examiners
(
    examiner_id serial NOT NULL references teachers,
    attestation_id serial NOT NULL references attestations,

    PRIMARY KEY (examiner_id, attestation_id)
);

CREATE TABLE student_attestations
(
    student_id serial NOT NULL references students,
    attestation_id serial NOT NULL references attestations,
    score integer NOT NULL
        CONSTRAINT students_attestations_score_check
            CHECK (score >= 0 AND score <= 100),
    -- rating_contributor_id serial references persons,

    PRIMARY KEY (student_id, attestation_id)
);

create table class_kinds
(
  name varchar(256) primary key
);

CREATE TABLE classes
(
    id serial
        PRIMARY KEY,
    curriculum_module_id serial NOT NULL references curriculum_modules,
    kind_name varchar(256) not null references class_kinds,
    duration_in_hours float NOT NULL
        CHECK (duration_in_hours > 0)
);

create table teacher_classes
(
  teacher_id serial not null references teachers,
  class_id serial not null references classes,
  study_group_id serial not null references study_groups,

  primary key (teacher_id, class_id, study_group_id)
);

create table class_teachers
(
  id serial primary key,

  teacher_id serial not null references teachers,
  class_id serial not null references classes,
  study_group_id serial not null references study_groups,

  unique (teacher_id, class_id, study_group_id)
);
