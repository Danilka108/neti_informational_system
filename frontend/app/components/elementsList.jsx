import { ListItemButton, Stack, Typography, List, Divider, Box, Paper, Button } from "@mui/material";
import React, { useRef, useState } from "react";
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

  const contentRef = useRef(null);
  const [printed, setPrinted] = useState(false);


  const handlePrint = (event) => {
    const printContents = contentRef.current.innerHTML;
    const iframe = document.createElement('iframe');

    document.body.appendChild(iframe);

    iframe.contentDocument.write(`
      <html>
        <head>
          <title>Print</title>
        </head>
        <body>${printContents}</body>
      </html>
    `);

    iframe.contentWindow.print();

    // Ждем некоторое время перед удалением iframe
    setTimeout(() => {
      document.body.removeChild(iframe);
      setPrinted(true);
    }, 1000);
  }

  return (
    <Stack direction="row" height="100%" style={{ width: "100%" }}>
      <Stack>
        <Button onClick={handlePrint}>print</Button>
        <Typography sx={{ whiteSpace: "nowrap", padding: "0.5em 1.5em 0.5em 1.5em", fontWeight: "normal", display: "flex", flexDirection: "row", justifyContent: "left", alignItems: "left" }} variant="subtitle1">
          {label}
        </Typography>
        <Divider />
        <List sx={{ padding: "0px" }}>
          {list_items}
        </List>
      </Stack>
      <Paper ref={contentRef} style={{ width: "100%", height: "100%", overflow: "clip", display: "flex", flexDirection: "row", alignItems: "center", justifyContent: "center" }}>
        <Outlet key={defaultId} />
      </Paper>
    </Stack>
  )
}
