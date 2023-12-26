import { ListItemButton, Stack, Typography, List, Divider, Box, Paper } from "@mui/material";
import { useParams } from "@remix-run/react";
import React, { useState } from "react";
import { Link, Outlet } from "react-router-dom";
import ElementsList from "../../components/elementsList";
import { API_HOST } from "../../root";

const subdivisions = [
  {
    id: 0,
    name: "asu",
  },
  {
    id: 1,
    name: "avtf",
  },
]

export const loader = async () => {
  const response = await fetch(API_HOST + `/subdivisions`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data

}

export default function StudyGroups() {
  const { subdivisionId } = useParams();

  return (<ElementsList defaultId={subdivisionId} elements={subdivisions} label="subdivisions" />)
}
