import { ListItemButton, Stack, Typography, List, Divider, Box, Paper } from "@mui/material";
import { useParams } from "@remix-run/react";
import React, { useState } from "react";
import { Link, Outlet } from "react-router-dom";
import ElementsList from "../../components/elementsList";

const curriculums = [
  {
    id: 0,
    name: "asu",
  },
  {
    id: 1,
    name: "avtf",
  }
];

export default function Curriculums() {
  const { curriculumId } = useParams();

  return (<ElementsList defaultId={curriculumId} elements={curriculums} label="curriculums" />)
}
