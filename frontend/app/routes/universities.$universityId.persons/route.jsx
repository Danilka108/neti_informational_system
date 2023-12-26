import { ListItemButton, Stack, Typography, List, Divider, Box, Paper } from "@mui/material";
import { useLoaderData, useParams } from "@remix-run/react";
import React, { useState } from "react";
import { Link, Outlet } from "react-router-dom";
import ElementsList from "../../components/elementsList";
import { API_HOST } from "../../root";

// const persons = [
//   {
//     id: 0,
//     name: "danil churickov igorevich",
//   },
//   {
//     id: 1,
//     name: "artem pronko vyacheslavovich",
//   },
// ];

export const loader = async ({ params }) => {
  const response = await fetch(API_HOST + `/persons`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const data = await response.json();
  return data

}

export default function Curriculums() {
  const persons = useLoaderData();
  const { personId } = useParams();

  return (<ElementsList defaultId={personId} elements={persons} label="persons" />)
}
