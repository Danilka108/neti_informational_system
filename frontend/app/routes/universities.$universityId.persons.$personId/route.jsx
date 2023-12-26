import { Divider, List, ListItem, ListItemButton, Paper, Stack, Table, TableBody, TableCell, TableHead, TableRow, Typography } from "@mui/material";
import { Link, json, useLoaderData, useParams } from "@remix-run/react";
import { API_HOST } from "../../root";

// const person = {
//   fullName: "danil churickov igorevich",
//   roles: [
//     {
//       role: "student",
//       studyGroupId: 0,
//       studyGroupName: "avt-113",
//     },
//     {
//       role: "teacher",
//       departmentId: 0,
//       departmentName: "asu",
//     },
//     {
//       role: "subdivisionMember",
//       subdivisionId: 0,
//       subdivisionRole: "asdfasf",
//       subdivisionName: "avtf",
//     },
//   ]
// }

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST + `/persons/${params.personId}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data
}

function StudentRole({ student, index }) {
  const { universityId } = useParams();

  const to = `/universities/${universityId}/study_groups/${student.studyGroupId}`;

  return (
    <Stack key={index} direction="column">
      <Typography key={index} style={{ padding: "0px", paddingTop: "10px", width: "100%", textAlign: "left" }}>
        student
      </Typography>
      <Typography key={index} color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
        study group {student.studyGroupName}
      </Typography>
    </Stack>
  );
}

function TeacherRole({ teacher, index }) {
  const { universityId } = useParams();

  const to = `/universities/${universityId}/subdivisions/${teacher.departmentId}`;

  return (
    <Stack direction="column" key={index}>
      <Typography key={index} style={{ padding: "0px", paddingTop: "10px", width: "100%", textAlign: "left" }}>
        teacher
      </Typography>
      <Typography key={index} color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
        department {teacher.departmentName}
      </Typography>
    </Stack>
  );
}

function SubdivisionMemberRole({ member, index }) {
  const { universityId } = useParams();

  const to = `/universities/${universityId}/subdivisions/${member.subdivisionId}`;

  return (
    <Stack key={index} direction="column">
      <Typography key={index} style={{ padding: "0px", paddingTop: "10px", width: "100%", textAlign: "left" }}>
        subdivision member
      </Typography>
      <Typography key={index} color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
        subdivision {member.subdivisionName}
      </Typography>
    </Stack>
  );
}

function Role({ role, index }) {
  if (role.role == "student") {
    return <StudentRole key={index} student={role} index={index} />
  }

  if (role.role == "subdivisionMember") {
    return <SubdivisionMemberRole key={index} member={role} index={index} />
  }

  if (role.role == "teacher") {
    return <TeacherRole key={index} teacher={role} index={index} />
  }

  return (
    <Box />
  );
}

export default function SelectedStudyGroup() {
  const data = useLoaderData();

  const roles = data.roles.map((item, index) => {
    return <Role key={index} role={item} index={index} />;
  })

  return (
    <Stack direction="column" style={{ paddingLeft: "20px", paddingTop: "20px", height: "100%", overflow: "auto", width: "100%" }}>
      <Typography variant="h6" style={{ width: "100%", textAlign: "left" }}>
        full name: "{data.fullName}"
      </Typography>
      <Divider />
      <Typography style={{ padding: "0px", paddingTop: "10px", width: "100%", textAlign: "left" }}>
        roles:
      </Typography>
      {roles}
    </Stack>
  )
}
