import { Box, Divider, List, ListItemButton, Stack, Typography } from "@mui/material";
import { Link, Outlet, json, useLoaderData, useParams } from "@remix-run/react";
import React, { useState } from "react";
export default function University() {
  const { universityId } = useParams();
  const [selectedIndex, setSelectedIndex] = useState(universityId || -1);

  const handleListItemClick = (
    event,
    index,
  ) => {
    setSelectedIndex(index);
  };

  const universities = [
    {
      id: 0,
      name: "University one"
    },
    {
      id: 1,
      name: "University two"
    },
  ];


  const items = universities.map((item, index) => (
    <Box>
      <ListItemButton key={index} onClick={(event) => handleListItemClick(event, index)} selected={index == selectedIndex} component={Link} to={`${item.id}`}>
        <Typography sx={{ fontWeight: "normal", textTransform: "lowercase" }} variant="subtitle2">
          {item.name}
        </Typography>
      </ListItemButton>
    </Box >
  ));

  return (
    <Box key={universityId} style={{ height: "100vh" }}>
      <Outlet style={{ height: "100vh" }} />
    </Box>
    // <div >
    //   <Stack height="100%" display="flex" direction="column">
    //     {/* <Stack> */}
    //     {/*   <Box style={{ height: "44px" }} /> */}
    //     {/*   <Typography variant="subtitle1" sx={{ padding: "0", paddingTop: "10px", display: "flex", justifyContent: "left", alignItems: "center", textAlign: "center" }}> */}
    //     {/*     universities: */}
    //     {/*   </Typography> */}
    //     {/*   <Divider /> */}
    //     {/*   <List sx={{ margin_top: 0, padding: "0" }}> */}
    //     {/*     {items} */}
    //     {/*   </List> */}
    //     {/* </Stack> */}
    //     <Divider />
    //   </Stack>
    // </div >
  );
}
