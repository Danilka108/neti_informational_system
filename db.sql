DROP SCHEMA IF EXISTS public CASCADE;
CREATE SCHEMA public;

CREATE DOMAIN seconds_from_unix_epoch bigint CHECK (value > 0);

CREATE TYPE gender AS ENUM ('MALE', 'FEMALE');

CREATE FUNCTION is_numeric(text) RETURNS boolean AS
    'SELECT $1 ~ ''^[0-9]+$'' ' LANGUAGE 'sql';

CREATE TYPE user_role AS ENUM ('ADMIN');

CREATE TYPE teacher_kind AS enum ('assistant', 'regular_teacher', 'senior_teacher', 'associate_professor', 'professor');

CREATE TYPE qualification AS enum ('bachelor', 'master', 'postgraduate', 'doctorate');

CREATE TYPE faculty_kind AS enum ('faculty', 'institute');

CREATE TYPE department_kind AS enum ('chair', 'department');

CREATE TYPE training_kind AS enum ('FULL_TIME', 'CORRESPONDENCE');

CREATE TYPE attestation_kind AS enum ('TEST', 'DIFF_TEST', 'EXAM');

CREATE TYPE five_point_rating AS enum ('0', '1', '2', '3', '4', '5', 'passed');

CREATE TYPE ects_rating AS enum ('A', 'B', 'C', 'D', 'E', 'FX');

CREATE TYPE class_kind AS enum ('LECTURE', 'SEMINAR_OR_PRACTISE', 'LAB_WORK', 'CONSULTATION', 'COURSE_WORK', 'COURSE_PROJECT', 'CALCULATION_AND_GRAPHICALLY_TASK_OR_ESSAY', 'VALIDATION_WORK');

CREATE TABLE users
(
    id serial NOT NULL
        PRIMARY KEY,
    email varchar(255) NOT NULL
        UNIQUE,
    role user_role NOT NULL,
    hashed_password text NOT NULL
);

CREATE TABLE user_sessions
(
    user_id serial NOT NULL,
    metadata varchar(1024) NOT NULL,
    refresh_token varchar(1024) NOT NULL
        UNIQUE,
    expires_at_in_seconds integer NOT NULL,

    PRIMARY KEY (user_id, metadata)
);


CREATE TABLE persons
(
    id serial
        PRIMARY KEY
);


create table universities
(
  id serial primary key,
  name varchar(256) not null UNIQUE
);

create table subdivisions
(
  id serial primary key,
  university_id serial not null references universities,
  name varchar(256) not null,

  unique (university_id, name)
);

create table tags
(
  id serial primary key,
  name varchar(128) unique not null
);

create table subdivisions_tags
(
  subdivision_id serial not null references subdivisions,
  tag_id serial not null references tags,

  unique (subdivision_id, tag_id)
);

create table subdivision_member
(
  subdivision_id serial NOT NULL REFERENCES subdivisions,
  member_id serial not null references persons,
  role varchar(512) not null,

  unique (subdivision_id, member_id)
);

create table study_groups
(
  id serial primary key,
  name varchar(256) not null unique,
  department_id serial not null references subdivisions
);

create table students
(
  id serial primary key references persons,
);

create table study_groups_students
(
  study_group_id serial not null references study_groups,
  student_id serial not null references students,
  studying_qualification qualification not null

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

create table disciplines
(
  id serial primary key,
  name varchar(256) not null,
  department_id serial not null references subdivisions
);

create table curriculum_items
(
  id serial primary key,
  curriculum_id serial not null references curriculums,
  discipline_id serial not null references disciplines,
  semester integer NOT NULL CHECK (semester > 0)
);

CREATE TABLE attestations
(
  id serial primary key,
  curriculum_item_id serial not null references curriculum_items,
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
            CHECK (score > 0),
    five_point_rating five_point_rating NOT NULL,
    ects_rating ects_rating NOT NULL,
    rating_contributor_id serial,

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
    curriculum_item_id serial NOT NULL references curriculum_items,
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

-- CREATE TABLE passports
-- (
--     id serial
--         PRIMARY KEY,
--     person_id serial NOT NULL,
--     issue_date date NOT NULL,
--     name varchar(255) NOT NULL,
--     surname varchar(255) NOT NULL,
--     patronymic varchar(255) NOT NULL,
--     gender gender NOT NULL,
--     birth_date date NOT NULL,
--     series char(4)
--         CHECK (is_numeric(series)),
--     number char(6)
--         CHECK (is_numeric(number))
-- );

-- CREATE TABLE teachers
-- (
--     id serial
--         PRIMARY KEY,
--     kind teacher_kind NOT NULL,
--     department_id serial NOT NULL
-- );

-- CREATE TABLE students
-- (
--     id serial
--         PRIMARY KEY,
--     studying_qualification qualification,
--     department_id serial NOT NULL
-- );

-- CREATE TABLE scientific_councils
-- (
--     id serial NOT NULL
--         PRIMARY KEY,
--     head_id serial NOT NULL
-- );

-- CREATE TABLE scientific_council_employees
-- (
--     employee_id serial
--         PRIMARY KEY,
--     scientific_council_id serial NOT NULL,
--     role varchar(200) NOT NULL
-- );

-- CREATE TABLE deans_offices
-- (
--     id serial
--         PRIMARY KEY,
--     dean_id serial NOT NULL,
--     vise_dean_id serial NOT NULL
-- );

-- CREATE TABLE deans_office_employees
-- (
--     employee_id serial
--         PRIMARY KEY,
--     deans_office_id serial NOT NULL,
--     field_of_activity text NOT NULL
-- );

-- CREATE TABLE rectorates
-- (
--     id serial
--         PRIMARY KEY,
--     rector_id serial NOT NULL
-- );

-- CREATE TABLE rectorate_employees
-- (
--     employee_id serial
--         PRIMARY KEY,
--     rectorate_id serial NOT NULL,
--     role varchar(255) NOT NULL,
--     field_of_activity text NOT NULL
-- );

-- CREATE TABLE universities
-- (
--     id serial
--         PRIMARY KEY,
--     name varchar(100)
--         UNIQUE,
--     scientific_council_id serial NOT NULL,
--     rectorate_id serial NOT NULL
-- );

-- CREATE TABLE faculties
-- (
--     id serial
--         PRIMARY KEY,
--     name varchar(100) NOT NULL,
--     university_id serial NOT NULL,
-- --         UNIQUE (name, university_id),
--     kind faculty_kind NOT NULL,
--     deans_office_id serial NOT NULL,
--     scientific_council_id serial NOT NULL
-- );

-- CREATE TABLE departments
-- (
--     id serial
--         PRIMARY KEY,
--     name varchar(100) NOT NULL,
--     faculty_id serial NOT NULL,
-- --         UNIQUE (name, university_id)
--     kind department_kind NOT NULL,
--     head_id serial NOT NULL
-- );

-- CREATE TABLE curriculums
-- (
--     id serial
--         PRIMARY KEY,
--     university_id serial NOT NULL,
--     training_kind training_kind NOT NULL,
--     training_time_in_years float NOT NULL
--         CHECK (training_time_in_years > 0),
--     qualification qualification NOT NULL
-- );

-- CREATE TABLE subjects
-- (
--     id serial
--         PRIMARY KEY,
--     university_id serial NOT NULL,
--     name varchar(100) NOT NULL,
--     teaching_department_id serial NOT NULL
-- );

-- CREATE TABLE curriculum_items
-- (
--     id serial
--         PRIMARY KEY,
--     curriculum_id serial NOT NULL,
--     subject_id serial NOT NULL,
--     semester integer NOT NULL
--         CHECK (semester > 0),
--     course integer NOT NULL
--         CHECK (course > 0)
-- );

-- CREATE TABLE attestations
-- (
--     curriculum_item_id serial
--         PRIMARY KEY,
--     kind attestation_kind NOT NULL,
--     duration_in_hours float NOT NULL
--         CHECK (duration_in_hours > 0)
-- );

-- CREATE TABLE attestations_examiners
-- (
--     examiners_id serial NOT NULL,
--     attestation_id serial NOT NULL,

--     PRIMARY KEY (examiners_id, attestation_id)
-- );

-- CREATE TABLE students_attestations
-- (
--     student_id serial NOT NULL,
--     attestation_id serial NOT NULL,
--     score integer NOT NULL
--         CONSTRAINT students_attestations_score_check
--             CHECK (score > 0),
--     five_point_rating five_point_rating NOT NULL,
--     ects_rating ects_rating NOT NULL,
--     rating_contributor_id serial,

--     PRIMARY KEY (student_id, attestation_id)
-- );

-- CREATE TABLE classes
-- (
--     id serial
--         PRIMARY KEY,
--     curriculum_item_id serial NOT NULL,
--     kind class_kind NOT NULL,
--     duration_in_hours float NOT NULL
--         CHECK (duration_in_hours > 0)
-- );

-- CREATE TABLE teachers_classes
-- (
--     teacher_id serial NOT NULL,
--     class_id serial NOT NULL,

--     PRIMARY KEY (class_id, teacher_id)
-- );

-- CREATE TABLE study_groups
-- (
--     id serial
--         PRIMARY KEY,
--     name varchar(100) NOT NULL,
--     department_id serial NOT NULL,
--     curriculum_id serial NOT NULL
-- );

-- CREATE TABLE study_group_students
-- (
--     student_id serial NOT NULL,
--     study_group_id serial NOT NULL,
--     PRIMARY KEY (student_id, study_group_id)
-- );

-- CREATE TABLE qualification_works
-- (
--     id serial
--         PRIMARY KEY,
--     title varchar(255) NOT NULL
--         UNIQUE,
--     author_id serial NOT NULL,
--     qualification qualification NOT NULL
-- );

-- CREATE TABLE scientific_directions
-- (
--     id serial
--         PRIMARY KEY,
--     name varchar(100) NOT NULL,
--     head_id serial
--         UNIQUE,
--     university_id serial NOT NULL
-- );

-- CREATE TABLE students_scientific_directions
-- (
--     student_id serial NOT NULL,
--     direction_id serial NOT NULL,

--     PRIMARY KEY (student_id, direction_id)
-- );

-- CREATE TABLE teachers_scientific_directions
-- (
--     teacher_id serial NOT NULL,
--     direction_id serial NOT NULL,

--     PRIMARY KEY (teacher_id, direction_id)
-- );

-- CREATE TABLE scientific_themes
-- (
--     id serial
--         PRIMARY KEY,
--     name varchar(100) NOT NULL,
--     head_id serial NOT NULL,
--     university_id serial NOT NULL
-- );

-- CREATE TABLE students_scientific_themes
-- (
--     student_id serial NOT NULL,
--     theme_id serial NOT NULL,

--     PRIMARY KEY (student_id, theme_id)
-- );

-- CREATE TABLE teachers_scientific_themes
-- (
--     teacher_id serial NOT NULL,
--     theme_id serial NOT NULL,

--     PRIMARY KEY (teacher_id, theme_id)
-- );

-- CREATE TABLE laboratories
-- (
--     id serial
--         PRIMARY KEY,
--     faculty_id serial NOT NULL,
--     name varchar(100) NOT NULL,
--     head_id serial NOT NULL
-- );

-- ALTER TABLE users ADD CONSTRAINT user_id_fk
--     FOREIGN KEY (id) REFERENCES persons ON DELETE CASCADE;

-- ALTER TABLE user_sessions ADD CONSTRAINT user_session_user_id_fk
--     FOREIGN KEY (user_id) REFERENCES users ON DELETE CASCADE;

-- ALTER TABLE passports ADD CONSTRAINT passport_person_id_fk
--     FOREIGN KEY (person_id) REFERENCES persons ON DELETE CASCADE;

-- ALTER TABLE teachers ADD CONSTRAINT teacher_id_fk
--     FOREIGN KEY (id) REFERENCES persons ON DELETE CASCADE,
--                      ADD CONSTRAINT teacher_department_id_fk
--                          FOREIGN KEY (department_id) REFERENCES departments ON DELETE RESTRICT;

-- ALTER TABLE students ADD CONSTRAINT student_id_fk
--     FOREIGN KEY (id) REFERENCES persons ON DELETE CASCADE,
--                      ADD CONSTRAINT student_department_id_fk
--                          FOREIGN KEY (department_id) REFERENCES departments ON DELETE RESTRICT;

-- ALTER TABLE scientific_councils ADD CONSTRAINT scientific_council_head_id_fk
--     FOREIGN KEY (head_id) REFERENCES teachers ON DELETE RESTRICT;

-- ALTER TABLE scientific_council_employees
--     ADD CONSTRAINT scientific_council_employee_employee_id_fk
--         FOREIGN KEY (employee_id) REFERENCES persons ON DELETE CASCADE,
--     ADD CONSTRAINT scientific_council_employee_scientific_council_id_fk
--         FOREIGN KEY (scientific_council_id) REFERENCES scientific_councils ON DELETE RESTRICT;

-- ALTER TABLE deans_offices ADD CONSTRAINT deans_office_dean_id_fk
--     FOREIGN KEY (dean_id) REFERENCES teachers ON DELETE RESTRICT,
--                           ADD CONSTRAINT deans_office_vise_dean_id_fk
--                               FOREIGN KEY (vise_dean_id) REFERENCES teachers ON DELETE RESTRICT;

-- ALTER TABLE deans_office_employees
--     ADD CONSTRAINT deans_office_employee_employee_id_fk
--         FOREIGN KEY (employee_id) REFERENCES persons ON DELETE CASCADE,
--     ADD CONSTRAINT deans_office_employee_deans_office_id_fk
--         FOREIGN KEY (deans_office_id) REFERENCES deans_offices ON DELETE RESTRICT;

-- ALTER TABLE rectorates
--     ADD CONSTRAINT rectorate_rector_id_fk
--         FOREIGN KEY (rector_id) REFERENCES persons ON DELETE RESTRICT;

-- ALTER TABLE rectorate_employees
--     ADD CONSTRAINT rectorate_employee_employee_id_fk
--         FOREIGN KEY (employee_id) REFERENCES persons ON DELETE CASCADE,
--     ADD CONSTRAINT rectorate_employee_rectorate_id_fk
--         FOREIGN KEY (rectorate_id) REFERENCES rectorates ON DELETE RESTRICT;

-- ALTER TABLE universities
--     ADD CONSTRAINT university_scientific_council_id_fk
--         FOREIGN KEY (scientific_council_id) REFERENCES scientific_councils ON DELETE RESTRICT,
--     ADD CONSTRAINT university_rectorate_id_fk
--         FOREIGN KEY (rectorate_id) REFERENCES rectorates ON DELETE RESTRICT;

-- ALTER TABLE faculties
--     ADD CONSTRAINT faculty_university_id_fk
--         FOREIGN KEY (university_id) REFERENCES universities ON DELETE CASCADE,
--     ADD CONSTRAINT faculty_deans_office_id_fk
--         FOREIGN KEY (deans_office_id) REFERENCES deans_offices ON DELETE RESTRICT,
--     ADD CONSTRAINT faculty_scientific_council_id_fk
--         FOREIGN KEY (scientific_council_id) REFERENCES scientific_councils ON DELETE RESTRICT;

-- ALTER TABLE departments
--     ADD CONSTRAINT department_faculty_id_fk
--         FOREIGN KEY (faculty_id) REFERENCES faculties ON DELETE CASCADE,
--     ADD CONSTRAINT department_head_id_fk
--         FOREIGN KEY (head_id) REFERENCES teachers ON DELETE RESTRICT;

-- ALTER TABLE curriculums
--     ADD CONSTRAINT curriculums_university_id_fk FOREIGN KEY (university_id) REFERENCES universities ON DELETE CASCADE;

-- ALTER TABLE subjects
--     ADD CONSTRAINT subject_university_id_fk FOREIGN KEY (university_id) REFERENCES universities ON DELETE CASCADE,
--     ADD CONSTRAINT subject_teaching_department_id_fk
--         FOREIGN KEY (teaching_department_id) REFERENCES departments ON DELETE RESTRICT;

-- ALTER TABLE curriculum_items
--     ADD CONSTRAINT curriculum_items_curriculum_id_fk
--         FOREIGN KEY (curriculum_id) REFERENCES curriculums ON DELETE CASCADE,
--     ADD CONSTRAINT curriculum_items_subject_id_fk
--         FOREIGN KEY (subject_id) REFERENCES subjects ON DELETE RESTRICT;

-- ALTER TABLE attestations
--     ADD CONSTRAINT attestation_curriculum_item_id_fk
--         FOREIGN KEY (curriculum_item_id) REFERENCES curriculum_items ON DELETE CASCADE;

-- ALTER TABLE attestations_examiners
--     ADD CONSTRAINT attestations_examiners_examiner_id_fk
--         FOREIGN KEY (examiners_id) REFERENCES teachers ON DELETE CASCADE,
--     ADD CONSTRAINT attestations_examiners_attestation_id_fk
--         FOREIGN KEY (attestation_id) REFERENCES attestations ON DELETE RESTRICT;

-- ALTER TABLE students_attestations
--     ADD CONSTRAINT students_attestations_student_id_fk
--         FOREIGN KEY (student_id) REFERENCES students ON DELETE CASCADE,
--     ADD CONSTRAINT students_attestations_attestation_id_fk
--         FOREIGN KEY (attestation_id) REFERENCES attestations ON DELETE RESTRICT,
--     ADD CONSTRAINT students_attestations_rating_contributor_id_fk
--         FOREIGN KEY (rating_contributor_id) REFERENCES deans_office_employees ON DELETE SET NULL;

-- ALTER TABLE classes
--     ADD CONSTRAINT classes_curriculum_item_id_fk
--         FOREIGN KEY (curriculum_item_id) REFERENCES curriculum_items ON DELETE CASCADE;

-- ALTER TABLE teachers_classes
--     ADD CONSTRAINT teachers_classes_teacher_id_fk
--         FOREIGN KEY (teacher_id) REFERENCES persons ON DELETE RESTRICT,
--     ADD CONSTRAINT teachers_classes_class_id_fk
--         FOREIGN KEY (class_id) REFERENCES classes ON DELETE CASCADE;

-- ALTER TABLE study_groups ADD CONSTRAINT study_groups_department_id_fk
--     FOREIGN KEY (department_id) REFERENCES departments ON DELETE CASCADE,
--                          ADD CONSTRAINT study_groups_curriculum_id_fk
--                              FOREIGN KEY (curriculum_id) REFERENCES curriculums ON DELETE RESTRICT;

-- ALTER TABLE study_group_students
--     ADD CONSTRAINT study_group_students_student_id_fk
--         FOREIGN KEY (student_id) REFERENCES students ON DELETE CASCADE,
--     ADD CONSTRAINT study_group_study_students_study_group_id_fk
--         FOREIGN KEY (study_group_id) REFERENCES study_groups ON DELETE CASCADE;

-- ALTER TABLE qualification_works
--     ADD CONSTRAINT qualification_works_author_id_fk
--         FOREIGN KEY (author_id) REFERENCES persons ON DELETE CASCADE;

-- ALTER TABLE scientific_directions
--     ADD CONSTRAINT scientific_directions_university_id_fk
--         FOREIGN KEY (university_id) REFERENCES universities ON DELETE CASCADE,
--     ADD CONSTRAINT scientific_directions_head_id_fk
--     FOREIGN KEY (head_id) REFERENCES teachers ON DELETE RESTRICT;

-- ALTER TABLE students_scientific_directions
--     ADD CONSTRAINT students_scientific_directions_student_id_fk
--         FOREIGN KEY (student_id) REFERENCES students ON DELETE CASCADE,
--     ADD CONSTRAINT students_scientific_directions_direction_id_fk
--         FOREIGN KEY (direction_id) REFERENCES scientific_directions ON DELETE CASCADE;

-- ALTER TABLE teachers_scientific_directions
--     ADD CONSTRAINT teachers_scientific_directions_teacher_id_fk
--         FOREIGN KEY (teacher_id) REFERENCES teachers ON DELETE CASCADE,
--     ADD CONSTRAINT teachers_scientific_directions_direction_id_fk
--         FOREIGN KEY (direction_id) REFERENCES scientific_directions ON DELETE CASCADE;

-- ALTER TABLE scientific_themes
--     ADD CONSTRAINT scientific_themes_university_id_fk
--         FOREIGN KEY (university_id) REFERENCES universities ON DELETE CASCADE,
--     ADD CONSTRAINT scientific_themes_head_id_fk
--         FOREIGN KEY (head_id) REFERENCES teachers ON DELETE RESTRICT;

-- ALTER TABLE students_scientific_themes
--     ADD CONSTRAINT students_scientific_themes_student_id_fk
--         FOREIGN KEY (student_id) REFERENCES students ON DELETE CASCADE,
--     ADD CONSTRAINT students_scientific_themes_theme_id_fk
--         FOREIGN KEY (theme_id) REFERENCES scientific_themes ON DELETE RESTRICT;

-- ALTER TABLE teachers_scientific_themes
--     ADD CONSTRAINT teachers_scientific_themes_teacher_id_fk
--         FOREIGN KEY (teacher_id) REFERENCES teachers ON DELETE CASCADE,
--     ADD CONSTRAINT teachers_scientific_themes_theme_id_fk
--         FOREIGN KEY (theme_id) REFERENCES scientific_themes ON DELETE RESTRICT;

-- ALTER TABLE laboratories ADD CONSTRAINT laboratories_faculty_id_fk
--     FOREIGN KEY (faculty_id) REFERENCES faculties ON DELETE CASCADE;
