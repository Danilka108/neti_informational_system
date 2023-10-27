DROP SCHEMA IF EXISTS public CASCADE;
CREATE SCHEMA public;

CREATE TABLE persons
(
    id serial
        PRIMARY KEY
);

CREATE TABLE scientific_councils
(
    id serial NOT NULL
        PRIMARY KEY,
    head_id serial NOT NULL REFERENCES persons
);

CREATE TABLE scientific_council_employees
(
    employee_id serial
        PRIMARY KEY
        REFERENCES persons ON DELETE CASCADE,
    scientific_council_id serial NOT NULL
        REFERENCES scientific_councils ON DELETE RESTRICT,
    role varchar(200) NOT NULL,
);

CREATE TABLE deans_offices
(
    id serial
        PRIMARY KEY,
    dean_id serial NOT NULL
        REFERENCES persons ON DELETE RESTRICT,
    vise_dean_id serial NOT NULL
        REFERENCES persons ON DELETE RESTRICT
);

CREATE TABLE deans_office_employees
(
    employee_id serial
        PRIMARY KEY
        REFERENCES persons ON DELETE CASCADE,
    deans_office serial NOT NULL
        REFERENCES deans_offices ON DELETE RESTRICT,
    field_of_activity text NOT NULL
);

CREATE TABLE rectorates
(
    id serial
        PRIMARY KEY,
    rector_id serial NOT NULL
        REFERENCES persons ON DELETE RESTRICT
);

CREATE TABLE rectorate_employees
(
    employee_id serial
        PRIMARY KEY
        REFERENCES persons ON DELETE CASCADE,
    rectorate_id serial NOT NULL
        REFERENCES rectorates ON DELETE RESTRICT,
    role varchar(255) NOT NULL,
    field_of_activity text NOT NULL
);

CREATE TABLE universities
(
    id serial
        PRIMARY KEY,
    name varchar(100)
        UNIQUE,
    scientific_council_id serial NOT NULL
        REFERENCES scientific_councils ON DELETE RESTRICT,
    rectorate_id serial NOT NULL
        REFERENCES rectorates ON DELETE RESTRICT
);

CREATE TYPE faculty_kind AS enum ('faculty', 'institute');

CREATE TABLE faculties
(
    id serial
        PRIMARY KEY,
    name varchar(100) NOT NULL,
    university_id serial NOT NULL
        REFERENCES universities ON DELETE RESTRICT,
--         UNIQUE (name, university_id),
    kind faculty_kind NOT NULL,
    deans_office_id serial NOT NULL
        REFERENCES deans_offices ON DELETE RESTRICT,
    scientific_council_id serial NOT NULL
        REFERENCES scientific_councils ON DELETE RESTRICT
);

CREATE TYPE department_kind AS enum ('chair', 'department');

CREATE TABLE departments
(
    id serial
        PRIMARY KEY,
    name varchar(100) NOT NULL,
    faculty_id serial NOT NULL
        REFERENCES faculties ON DELETE RESTRICT,
--         UNIQUE (name, university_id)
    kind department_kind NOT NULL,
    head_id serial NOT NULL
        REFERENCES persons ON DELETE RESTRICT
);

CREATE TYPE teacher_kind AS enum ('assistant', 'regular_teacher', 'senior_teacher', 'associate_professor', 'professor');

CREATE TABLE teachers
(
    id serial
        PRIMARY KEY
        REFERENCES persons,
    kind teacher_kind NOT NULL,
    department_id serial NOT NULL
        REFERENCES departments
);

CREATE TYPE qualification AS enum ('bachelor', 'master', 'postgraduate', 'doctorate');

CREATE TABLE students
(
    id serial
        PRIMARY KEY
        REFERENCES persons ON DELETE CASCADE,
    studying_qualification qualification,
    departament_id serial NOT NULL REFERENCES departments
);

CREATE TYPE training_kind AS enum ('FULL_TIME', 'CORRESPONDENCE');

CREATE TABLE IF NOT EXISTS curriculums
(
    id serial
    PRIMARY KEY,
    training_kind training_kind NOT NULL,
    training_time_in_years float NOT NULL
    CHECK (training_time_in_years > 0),
    qualification qualification NOT NULL
    );

CREATE TABLE IF NOT EXISTS subjects
(
    id serial
      PRIMARY KEY,
    name varchar(100) NOT NULL,
    teaching_department_id serial NOT NULL
      REFERENCES departments ON DELETE RESTRICT
    );

CREATE TABLE IF NOT EXISTS curriculum_items
(
    id serial
      PRIMARY KEY,
    curriculum_id serial NOT NULL
      REFERENCES curriculums ON DELETE CASCADE,
    subject_id serial NOT NULL
      REFERENCES subjects ON DELETE RESTRICT,
    semester integer NOT NULL
      CHECK (semester > 0),
    course integer NOT NULL
      CHECK (course > 0)
    );

CREATE TYPE attestation_kind AS enum ('TEST', 'DIFF_TEST', 'EXAM');
CREATE TYPE five_point_rating AS enum ('0', '1', '2', '3', '4', '5', 'passed');
CREATE TYPE ects_rating AS enum ('A', 'B', 'C', 'D', 'E', 'FX');

CREATE TABLE attestations
(
    curriculum_item_id serial
        PRIMARY KEY
        REFERENCES curriculum_items ON DELETE RESTRICT,
    kind attestation_kind NOT NULL,
    duration_in_hours float NOT NULL
        CHECK (duration_in_hours > 0)
);

CREATE TABLE attestations_examiners
(
    examiners_id serial NOT NULL
        REFERENCES teachers ON DELETE RESTRICT,
    attestation_id serial NOT NULL
        REFERENCES attestations ON DELETE CASCADE,

    PRIMARY KEY (examiners_id, attestation_id)
);

CREATE TABLE students_attestations
(
    student_id serial NOT NULL
        REFERENCES students ON DELETE CASCADE,
    attestation_id serial NOT NULL
        REFERENCES attestations ON DELETE RESTRICT,
    score integer NOT NULL
        CONSTRAINT students_attestations_score_check
            CHECK (score > 0),
    five_point_rating five_point_rating NOT NULL,
    ects_rating ects_rating NOT NULL,
    rating_contributor_id serial
        REFERENCES deans_office_employees ON DELETE RESTRICT,

    PRIMARY KEY (student_id, attestation_id)
);

CREATE TYPE class_kind AS enum ('LECTURE', 'SEMINAR_OR_PRACTISE', 'LAB_WORK', 'CONSULTATION', 'COURSE_WORK', 'COURSE_PROJECT', 'CALCULATION_AND_GRAPHICALLY_TASK_OR_ESSAY', 'VALIDATION_WORK');

CREATE TABLE IF NOT EXISTS classes
(
    id serial
    PRIMARY KEY,
    curriculum_item_id serial NOT NULL
    REFERENCES curriculum_items ON DELETE CASCADE,
    kind class_kind NOT NULL,
    duration_in_hours float NOT NULL
    CHECK (duration_in_hours > 0)
    );

CREATE TABLE IF NOT EXISTS teachers_classes
(
    teacher_id serial NOT NULL
    REFERENCES teachers ON DELETE RESTRICT,
    class_id serial NOT NULL
    REFERENCES classes ON DELETE CASCADE,

    PRIMARY KEY (class_id, teacher_id)
    );

CREATE TABLE study_groups
(
    id serial
        PRIMARY KEY,
    name varchar(100) NOT NULL,
    department_id serial NOT NULL
        REFERENCES departments ON DELETE RESTRICT,
    curriculum_id serial NOT NULL REFERENCES curriculums
);

CREATE TABLE study_group_students
(
    students serial NOT NULL
        REFERENCES students ON DELETE CASCADE,
    study_group serial NOT NULL
        REFERENCES study_groups ON DELETE RESTRICT,
    PRIMARY KEY (students, study_group)
);

CREATE TABLE qualification_works
(
    id serial
        PRIMARY KEY,
    title varchar(255) NOT NULL
        UNIQUE,
    author_id serial NOT NULL
        REFERENCES persons,
    qualification qualification NOT NULL
);

CREATE TABLE scientific_directions
(
    id serial
        PRIMARY KEY,
    name varchar(100) NOT NULL,
    head_id serial
        UNIQUE
        REFERENCES teachers ON DELETE RESTRICT
);

CREATE TABLE students_scientific_directions
(
    student_id serial NOT NULL
        REFERENCES students ON DELETE CASCADE,
    scientific_direction_id serial NOT NULL
        REFERENCES scientific_directions ON DELETE RESTRICT,

    PRIMARY KEY (student_id, scientific_direction_id)
);

CREATE TABLE teachers_scientific_directions
(
    teacher_id serial NOT NULL
        REFERENCES teachers ON DELETE CASCADE,
    scientific_direction_id serial NOT NULL
        REFERENCES scientific_directions ON DELETE RESTRICT,

    PRIMARY KEY (teacher_id, scientific_direction_id)
);

CREATE TABLE scientific_themes
(
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL,
    head_id serial
        UNIQUE REFERENCES teachers ON DELETE SET NULL,
    scientific_direction_id serial NOT NULL
        REFERENCES scientific_directions ON DELETE RESTRICT
);

CREATE TABLE students_scientific_themes
(
    student_id serial NOT NULL
        REFERENCES students ON DELETE CASCADE,
    scientific_theme_id serial NOT NULL
        REFERENCES scientific_themes ON DELETE RESTRICT,

    PRIMARY KEY (student_id, scientific_theme_id)
);

CREATE TABLE teachers_scientific_themes
(
    teacher_id serial NOT NULL
        REFERENCES teachers ON DELETE CASCADE,
    scientific_theme_id serial NOT NULL
        REFERENCES scientific_themes ON DELETE RESTRICT,

    PRIMARY KEY (teacher_id, scientific_theme_id)
);

CREATE TABLE laboratories
(
    id serial
        PRIMARY KEY,
    faculty_id serial NOT NULL
        REFERENCES faculties ON DELETE RESTRICT,
    name varchar(100) NOT NULL,
    head_id serial NOT NULL
        REFERENCES teachers
);

CREATE TYPE user_role AS ENUM ('ADMIN');

CREATE TABLE users
(
    id serial NOT NULL
        PRIMARY KEY,
    person_id serial NOT NULL UNIQUE REFERENCES persons ON DELETE CASCADE,
    email varchar(255) NOT NULL
        UNIQUE,
    role user_role NOT NULL,
    hashed_password text NOT NULL
);

CREATE TABLE user_sessions
(
    user_id serial NOT NULL
        REFERENCES users ON DELETE CASCADE,
    metadata varchar(512) NOT NULL,

    refresh_token varchar(1024) NOT NULL
        UNIQUE,
    expires_at_in_seconds bigint NOT NULL
        CHECK (expires_at_in_seconds > 0),

    PRIMARY KEY (user_id, metadata)
);
