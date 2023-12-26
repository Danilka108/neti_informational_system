import { Divider, List, ListItem, ListItemButton, Paper, Stack, Table, TableBody, TableCell, TableHead, TableRow, Typography } from "@mui/material";
import { Link, json, useLoaderData, useParams } from "@remix-run/react";
import { API_HOST } from "../../root";

// const subdivision = {
//   name: "asu",
//   members: [
//     {
//       personId: 0,
//       fullName: "danil churickov",
//       role: "asfasf"
//     },
//     {
//       personId: 1,
//       fullName: "ahosdahfasdf",
//       role: "sdfsf",
//     },
//   ],
// }

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST + `/subdivisions/${params.subdivisionId}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data

}

export default function SelectedSubdivision() {
  const { universityId, subdivisionId } = useParams();
  const data = useLoaderData();

  const members = data.members.map((item, index) => {
    const to = `/universities/${universityId}/persons/${item.personId}`;

    return (
      <Typography color="blue" variant="subtitle1" component={Link} to={to} style={{ textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block" }}>
        {`${item.fullName} (role ${item.role})`}
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
        members:
      </Typography>
      <Stack width="fit-content">
        {members}
      </Stack>
    </Stack>
  )
}
