import { Box, Button, Grid, TextField, Typography } from "@mui/material";
import React, { useState } from "react";
import { API_HOST } from "../root";
import { useNavigate } from "@remix-run/react";

function checkPassword(password, setError) {
  if (password.length < 8) {
    setError('Password should be at least 8 characters long.');
    return;
  }

  if (!/[a-z]/.test(password) || !/[A-Z]/.test(password)) {
    setError('Password should contain at least one uppercase and one lowercase letter.');
    return;
  }
}

export default function Index() {
  const [error, setError] = useState(null);
  const navigate = useNavigate();

  const handleSubmit = async (event) => {
    event.preventDefault();
    const data = new FormData(event.currentTarget);

    const passport = data.get('password');
    const email = data.get("email");

    checkPassword(password, setError);

    const response = await fetch(API_HOST + "/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        password: passport,
        email: email,
      })
    });

    const json = await response.json();
    if (!response.ok) {
      setError(json.message);
    } else {
      navigate("/universities");
    }
  };

  return (
    <div style={{ height: "100%", display: "flex", flexDirection: "column", justifyContent: "center", alignItems: "center", fontFamily: "system-ui, sans-serif", lineHeight: "1.4" }}>
      <Box component="form" noValidate onSubmit={handleSubmit} sx={{ mt: 3, maxWidth: "300px", maxHeight: "300px", }}>
        <Grid container spacing={2}>
          <Grid item xs={12}>
            <TextField
              required
              fullWidth
              id="email"
              label="Email Address"
              name="email"
              autoComplete="email"
            />
          </Grid>
          <Grid item xs={12}>
            <TextField
              required
              fullWidth
              name="password"
              label="Password"
              type="password"
              id="password"
              autoComplete="new-password"
            />
          </Grid>
        </Grid>
        {
          (error != null) && <Box>
            <Typography color="red" variant="h6">{error}</Typography>
          </Box>
        }
        <Button
          type="submit"
          fullWidth
          variant="contained"
          sx={{ mt: 3, mb: 2 }}
        >
          login
        </Button>
      </Box>
    </div>
  );
}
