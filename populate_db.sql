delete from class_teachers;
delete from classes;
delete from class_kinds;
delete from student_attestations;
delete from attestation_examiners;
delete from attestations;
delete from curriculum_modules;
delete from disciplines;
delete from study_group_curriculums;
delete from curriculums;
delete from students;
delete from teachers;
delete from study_groups;
delete from subdivision_members;
delete from subdivision_tags;
delete from subdivisions;
delete from tags;
delete from universities;
delete from passports;
delete from persons;
delete from user_sessions;
delete from users;

insert into users (id, email, password) values (0, 'd.churikov@stud.nstu.ru', 'user');
insert into users (id, email, password) values (1, 'tomilov@corp.nstu.ru', 'user');
insert into users (id, email, password) values (2, 'reva@corp.nstu.ru', 'user');
insert into user_sessions (user_id, metadata, refresh_token, expires_at) values (0, 'chrome', 'token', 234234);
insert into persons (id, user_id, full_name) values (0, 0, 'danil churickov');
insert into persons (id, user_id, full_name) values (1, 1, 'tomilov ivan nokolaevich');
insert into persons (id, user_id, full_name) values (2, 2, 'reva ivan nikolaevich');
insert into passports (id, person_id, first_name, last_name, patronymic, date_of_birth, date_of_issue, number, series, gender)
  values (0, 0, 'danil', 'churikov', 'igorevich', '2002-12-31 22:00:00', '2022-03-01 12:00:00', '444444', '4444', 'male');

insert into universities (id, name) values (0, 'nstu');
insert into tags (name) values ('faculty');
insert into tags (name) values ('department');
insert into subdivisions (id, university_id, name) values (0, 0, 'asu');
insert into subdivisions (id, university_id, name) values (1, 0, 'avtf');
insert into subdivision_tags (tag_name, subdivision_id) values ('department', 0);
insert into subdivision_tags (tag_name, subdivision_id) values ('faculty', 1);
insert into subdivision_members (subdivision_id, person_id, role) values (0, 1, 'deputy department');
insert into subdivision_members (subdivision_id, person_id, role) values (1, 2, 'dean');

insert into study_groups (id, name, department_id, studying_qualification, training_kind) values (0, 'avt-113', 0, 'bachelor', 'full_time');
insert into teachers (id, person_id, kind, department_id) values (0, 1, 'associate_professor', 0);
insert into students (id, person_id, study_group_id) values (0, 0, 0);

insert into curriculums (id, name) values (0, 'avt-113 2021');
insert into study_group_curriculums (study_group_id, curriculum_id) values (0, 0);
insert into disciplines (id, department_id, name) values (0, 0, 'informatics');
insert into curriculum_modules (id, curriculum_id, discipline_id, semester) values (0, 0, 0, 1);

insert into attestations (id, curriculum_module_id, kind, duration_in_hours) values (0, 0, 'exam', 100);
insert into attestation_examiners (examiner_id, attestation_id) values (0, 0);
insert into student_attestations (student_id, attestation_id, score)
  values (0, 0, 99);

insert into class_kinds (name) values ('lection');
insert into classes (id, curriculum_module_id, kind_name, duration_in_hours) values (0, 0, 'lection', 2);
insert into class_teachers (teacher_id, class_id, study_group_id) values (0, 0, 0);
