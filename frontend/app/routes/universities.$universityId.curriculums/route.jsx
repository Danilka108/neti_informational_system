import { ListItemButton, Stack, Typography, List, Divider, Box, Paper } from "@mui/material";
import { useLoaderData, useParams } from "@remix-run/react";
import React, { useState } from "react";
import { Link, Outlet } from "react-router-dom";
import ElementsList from "../../components/elementsList";
import { API_HOST } from "../../root";

// const curriculums = [
//   {
//     id: 0,
//     name: "asu",
//   },
//   {
//     id: 1,
//     name: "avtf",
//   }
// ];

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST + `/curriculums`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data
}

export default function Curriculums() {
  const curriculums = useLoaderData();
  const { curriculumId } = useParams();

  return (<ElementsList defaultId={curriculumId} elements={curriculums} label="curriculums" />)
}
