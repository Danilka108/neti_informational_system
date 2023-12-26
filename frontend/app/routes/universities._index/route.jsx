import { Box, Divider, List, ListItemButton, Paper, Stack, Typography } from "@mui/material";
import { json, useLoaderData, Link } from "@remix-run/react";
import { API_HOST } from "../../root";

// const universities = [
//   {
//     universityName: "ngtu",
//     universityId: 0,
//   },
//   {
//     universityName: "ngu",
//     universityId: 1,
//   },
// ];

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST + "/universities", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return json(data)
}

export default function UnselectedUniversity() {
  const universities = useLoaderData();

  const items = universities.map((item, index) => {
    const to = `/universities/${item.universityId}`;
    return (
      <Stack key={index}>
        <ListItemButton component={Link} to={to}>
          <Typography sx={{ display: "flex", justifyContent: "left", alignItems: "center", textAlign: "center" }} variant="subtitle1">
            {item.universityName}
          </Typography>
        </ListItemButton>
        <Divider />
      </Stack >
    );
  });

  return (
    <Stack style={{ width: '100%', height: '100%', alignItems: 'center', justifyContent: "center", }}>
      <Typography variant="h5" align="center">
        Select university
      </Typography>
      <Paper sx={{ marginTop: "20px", }}>
        <List sx={{ minWidth: "150px", display: "flex", justifyContent: "left", alignItems: "left", textAlign: "center", flexDirection: "column" }}>
          <Divider key={-1} />
          {items}
        </List>
      </Paper>
    </Stack>
  )
}
