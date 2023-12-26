import { Divider, List, ListItem, ListItemButton, Paper, Stack, Table, TableBody, TableCell, TableHead, TableRow, Typography } from "@mui/material";
import { Link, json, useLoaderData, useParams } from "@remix-run/react";
import { API_HOST } from "../../root";

// const curriculum = {
//   name: "avt-113 avt-114 2021",
//   studyGroups: [
//     {
//       id: 0,
//       name: "avt-113"
//     },
//     {
//       id: 1,
//       name: "avt-114"
//     }
//   ],
//   semesters: [
//     {
//       value: 1,
//       modules: [
//         {
//           disciplineName: "informatics",
//           departmentName: "asu",
//           "departmentId": 0,
//         },
//         {
//           disciplineName: "programing",
//           departmentName: "asu",
//           departmentId: 0,
//         },
//       ]
//     },
//     {
//       value: 2,
//       modules: [
//         {
//           disciplineName: "networks",
//           departmentName: "someone else",
//           "departmentId": 1,
//         },
//       ]
//     },
//   ]

// }

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST + `/curriculums/${params.curriculumId}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data
};

export default function SelectedCurriculum() {
  const { universityId } = useParams();
  const data = useLoaderData();
  // console.log(data);
  // return <div></div>

  const studyGroups = data.studyGroups.map((item, index) => {
    const to = `/universities/${universityId}/study_groups/${item.id}`;
    return (
      <Typography key={index} color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
        {item.name}
      </Typography>
    );
  });

  const semesters = data.semesters.map((semester, semesterIndex) => {
    const rows = semester.modules.map((module, moduleIndex) => {
      const to = `/universities/${universityId}/subdivisions/${module.departmentId}`;

      return (
        <TableRow key={moduleIndex}>
          <TableCell>{module.disciplineName}</TableCell>
          <TableCell>
            <Typography key={moduleIndex} color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
              {module.departmentName}
            </Typography>
          </TableCell>
        </TableRow>
      );
    });

    return (
      <Stack key={semesterIndex} direction="column" sx={{ marginTop: "10px", maxWidth: "40em", }}>
        <Typography variant="h6">
          semester {semester.value}
        </Typography>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>discipline name</TableCell>
              <TableCell>teaching department</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {rows}
          </TableBody>
        </Table>
      </Stack>
    );
  });

  return (
    <Stack direction="column" style={{ paddingLeft: "20px", paddingTop: "20px", height: "100%", overflow: "auto", width: "100%" }}>
      {/* <Stack direction="column" style={{ height: "100%", overflow: "auto" }}> */}
      <Typography variant="h6" style={{ width: "100%", textAlign: "left" }}>
        name: "{data.name}"
      </Typography>
      <Divider />
      <Typography style={{ padding: "0px", paddingTop: "10px", width: "100%", textAlign: "left" }}>
        study groups:
      </Typography>
      <Stack width="fit-content">
        {studyGroups}
      </Stack>
      {semesters}
    </Stack>
  )
}
