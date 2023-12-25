import { Box, Tab, Typography, Tabs, Stack, Paper, Divider } from "@mui/material";
import { Link, Outlet, useMatches, useParams } from "@remix-run/react"
import { useState } from "react";

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

export default function University() {
  const { universityId } = useParams();
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
      <Box>
        <Typography variant="h6" sx={{ padding: "10px", textTransform: "uppercase", display: "flex", justifyContent: "left", alignItems: "center", textAlign: "center" }}>
          university "{universityId}"
        </Typography>
        <Divider />
      </Box>
      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <Tabs value={tabValue} onChange={handleTabChange}>
          {tabs_items}
        </Tabs>
      </Box>
      <Box sx={{ flex: "1", overflow: "scroll", }}>
        <Outlet />
      </Box>
    </Stack>
  )
}
