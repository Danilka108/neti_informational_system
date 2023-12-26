import { ListItemButton, Stack, Typography, List, Divider, Box, Paper } from "@mui/material";
import { useLoaderData, useParams } from "@remix-run/react";
import React, { useState } from "react";
import { Link, Outlet } from "react-router-dom";
import ElementsList from "../../components/elementsList";
import { API_HOST } from "../../root";

// const studyGroups = [
//   {
//     id: 0,
//     name: "AVT-112",
//   },
//   {
//     id: 1,
//     name: "AVT-113",
//   },
//   {
//     id: 2,
//     name: "AVT-114",
//   },
// ]

export const loader = async () => {
  const response = await fetch(API_HOST + `/study_groups`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data
}

export default function StudyGroups() {
  const studyGroups = useLoaderData();
  const { studyGroupId } = useParams();

  return (<ElementsList defaultId={studyGroupId} elements={studyGroups} label="study groups" />)
}
