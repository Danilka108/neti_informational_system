import { Box, Tab, Typography, Tabs, Stack, Paper, Divider, Button, Dialog, DialogTitle, DialogContent, DialogContentText, DialogActions } from "@mui/material";
import { Link, Outlet, useLoaderData, useMatches, useParams } from "@remix-run/react"
import { useState } from "react";
import { API_HOST } from "../../root";

const tabs = [
  {
    value: "subdivisions",
    label: "subdivisions",
  },
  {
    value: "study_groups",
    label: "study groups",
  },
  {
    value: "curriculums",
    label: "curriculums",
  },
  {
    value: "persons",
    label: "persons",
  },
];

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST + `/universities/${params.universityId}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data

}

export default function University() {
  const { name: universityName } = useLoaderData();
  const matches = useMatches();
  const { pathname } = matches[3];
  const element = pathname.split("/")[3];

  const [tabValue, setTabValue] = useState(element || false);

  const handleTabChange = (event, newValue) => {
    setTabValue(newValue);
  }

  const tabs_items = tabs.map((item, index) => {
    return (
      <Tab
        key={index}
        value={item.value}
        label={
          <Typography variant="subtitle2" align="center">
            {item.label}
          </Typography>
        }
        component={Link}
        to={item.value}
      />
    );
  });

  return (
    <Stack direction="column" sx={{ height: "100%", overflow: "clip", }}>
      <Stack direction="column" justifyContent="center">
        <Typography color="blue" variant="subtitle1" component={Link} to="/universities" style={{ paddingTop: "0", textTransform: "lowercase", padding: "0", textDecoration: "underline", display: "inline-block", marginLeft: "10px" }}>
          change university
        </Typography>
        <Typography variant="h6" sx={{ padding: "10px", paddingBottom: "0px", paddingTop: "0", textTransform: "uppercase", display: "flex", justifyContent: "left", alignItems: "center", textAlign: "center" }}>
          university "{universityName}"
        </Typography>
        <Divider />
      </Stack>
      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <Tabs value={tabValue} onChange={handleTabChange}>
          {tabs_items}
        </Tabs>
      </Box>
      <Box sx={{ flex: "1", overflow: "scroll", display: "flex", flexDirection: "row", alignItems: "center", justifyContent: "center", width: "100%" }}>
        <Outlet />
      </Box>
    </Stack>
  )
}
