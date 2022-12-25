import * as React from 'react'
import {
  Box,
  Button,
  Collapse,
  IconButton,
  InputAdornment,
  Paper,
  TextField,
  Typography,
} from '@mui/material'
import Visibility from '@mui/icons-material/Visibility'
import VisibilityOff from '@mui/icons-material/VisibilityOff'
import Grid2 from '@mui/material/Unstable_Grid2/Grid2'

import { useStatic } from '@hooks/useStatic'
import ThemeToggle from './ThemeToggle'

export default function Login() {
  const [password, setPassword] = React.useState<string>('')
  const [error, setError] = React.useState<string>('')
  const [show, setShow] = React.useState<boolean>(false)

  const setStatic = useStatic((s) => s.setStatic)

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const res = await fetch('/api/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ password }),
    })
    if (res.status === 200) {
      setStatic('loggedIn', true)
    } else {
      setError('Wrong Password')
    }
  }

  return (
    <Paper
      component={Grid2}
      sx={{
        display: 'flex',
        height: '100vh',
        width: '100%',
        flexDirection: 'column',
      }}
      square
      elevation={0}
      className="section-process"
    >
      <Grid2>
        <div className="section-container">
          <div className="process-steps-container container-medium with-padding">
            <div className="process-step-container process-step-1">
              <div className="process-step-title-container">
                <h1 className="process-step-title" style={{ color: 'black' }}>
                  Kōji
                </h1>
                <div className="process-step-title-overlay">Kōji</div>
              </div>
            </div>
          </div>
        </div>
      </Grid2>
      <Grid2 mt={8}>
        <form
          autoComplete="off"
          onSubmit={onSubmit}
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            flexDirection: 'column',
          }}
        >
          <TextField
            name="password"
            label="Password"
            type={show ? 'text' : 'password'}
            value={password}
            onChange={(e) => {
              setError('')
              setPassword(e.target.value)
            }}
            // color="inherit"
            error={!!error}
            InputProps={{
              endAdornment: (
                <InputAdornment position="end">
                  <IconButton
                    onClick={() => setShow((prev) => !prev)}
                    onMouseDown={(e) => e.preventDefault()}
                  >
                    {show ? <Visibility /> : <VisibilityOff />}
                  </IconButton>
                </InputAdornment>
              ),
            }}
          />
          <Button
            color={error ? 'error' : 'inherit'}
            type="submit"
            sx={{
              mt: 2,
            }}
          >
            Login
          </Button>
        </form>
        <Collapse in={!!error}>
          <Typography color="error" sx={{ mt: 2 }}>
            {error}
          </Typography>
        </Collapse>
      </Grid2>
      <Grid2 height="20vh" />
      <Box sx={{ position: 'absolute', top: 10, right: 10 }}>
        <ThemeToggle />
      </Box>
    </Paper>
  )
}