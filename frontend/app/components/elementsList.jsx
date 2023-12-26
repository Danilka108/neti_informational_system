import { ListItemButton, Stack, Typography, List, Divider, Box, Paper } from "@mui/material";
import React, { useState } from "react";
import { Link, Outlet } from "react-router-dom";


export default function ElementsList({ defaultId, elements, label }) {
  const [selectedId, setSelectedId] = useState(defaultId || -1);

  const list_items = elements.map((item, index) => {
    return (
      <Box key={index}>
        <ListItemButton onClick={(_) => setSelectedId(item.id)} selected={item.id == selectedId} component={Link} to={`${item.id}`}>
          <Typography sx={{ display: "flex", justifyContent: "left", alignItems: "center", textAlign: "center", whiteSpace: "nowrap" }} variant="subtitle1">
            {item.name}
          </Typography>
        </ListItemButton>
        <Divider />
      </Box>
    );
  })

  return (
    <Stack direction="row" height="100%" style={{ width: "100%" }}>
      <Stack>
        <Typography sx={{ whiteSpace: "nowrap", padding: "0.5em 1.5em 0.5em 1.5em", fontWeight: "normal", display: "flex", flexDirection: "row", justifyContent: "left", alignItems: "left" }} variant="subtitle1">
          {label}
        </Typography>
        <Divider />
        <List sx={{ padding: "0px" }}>
          {list_items}
        </List>
      </Stack>
      <Paper style={{ width: "100%", height: "100%", overflow: "clip", display: "flex", flexDirection: "row", alignItems: "center", justifyContent: "center" }}>
        <Outlet key={defaultId} />
      </Paper>
    </Stack>
  )
}
