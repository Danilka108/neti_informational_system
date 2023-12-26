
import { Divider, List, ListItem, ListItemButton, Paper, Stack, Table, TableBody, TableCell, TableHead, TableRow, Typography } from "@mui/material";
import { Link, json, useLoaderData, useParams } from "@remix-run/react";
import { API_HOST } from "../../root";

// const studyGroup = {
//   name: "avt-113",
//   curriculums: [
//     {
//       id: 0,
//       name: 'avt-113 avt-114 2021',
//     },
//     {
//       id: 1,
//       name: 'something else',
//     },
//   ],
//   students: [
//     {
//       personId: 0,
//       fullName: "danil churickov igorevich",
//     },
//     {
//       personId: 0,
//       fullName: "artem pronko vyacheslavovich",
//     },
//   ],
// }

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST
    + `/study_groups/${params.studyGroupId}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data
}

export default function SelectedStudyGroup() {
  const { universityId, studyGroupId } = useParams();
  const data = useLoaderData();

  const students = data.students.map((item, index) => {
    const to = `/universities/${universityId}/persons/${item.personId}`;
    return (
      <Typography key={item.personId} color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
        {item.fullName}
      </Typography>
    );
  });

  const curriculums = data.curriculums.map((item, index) => {
    const to = `/universities/${universityId}/curriculums/${item.id}`;
    return (
      <Typography key={item.id} color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
        {item.name}
      </Typography>
    );
  });

  return (
    <Stack direction="column" style={{ paddingLeft: "20px", paddingTop: "20px", height: "100%", overflow: "auto", width: "100%" }}>
      <Typography variant="h6" style={{ width: "100%", textAlign: "left" }}>
        name: "{data.name}"
      </Typography>
      <Divider />
      <Typography style={{ padding: "0px", paddingTop: "10px", width: "100%", textAlign: "left" }}>
        students:
      </Typography>
      <Stack width="fit-content">
        {students}
      </Stack>
      <Typography style={{ padding: "0px", paddingTop: "10px", width: "100%", textAlign: "left" }}>
        curriculums:
      </Typography>
      <Stack width="fit-content">
        {curriculums}
      </Stack>
    </Stack>
  )
}
