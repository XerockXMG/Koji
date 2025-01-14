import * as React from 'react'
import { CssBaseline, ThemeProvider } from '@mui/material'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'

import createTheme from '@assets/theme'
import type { Config } from '@assets/types'
import { usePersist } from '@hooks/usePersist'
import { useStatic } from '@hooks/useStatic'
import { fetchWrapper } from '@services/fetches'

import Home from '@pages/Home'
import Map from '@pages/map'
import AdminPanel from '@pages/admin'
import ErrorPage from '@pages/Error'
import Login from '@pages/Login'
import ConvertPage from '@pages/Convert'
import Playground from '@pages/Playground'

const router = createBrowserRouter([
  {
    path: '/',
    element: <Home />,
    errorElement: <ErrorPage error="500" />,
  },
  {
    path: '/login',
    element: <Login />,
    errorElement: <ErrorPage error="500" />,
  },
  {
    path: '/map',
    element: <Map />,
    errorElement: <ErrorPage error="500" />,
  },
  {
    path: '/map/:lat/:lon/:zoom?',
    element: <Map />,
    errorElement: <ErrorPage error="500" />,
  },
  {
    path: '/admin/*',
    element: <AdminPanel />,
    errorElement: <ErrorPage error="500" />,
  },
  {
    path: '/convert',
    element: <ConvertPage />,
    errorElement: <ErrorPage error="500" />,
  },
  {
    path: '/play',
    element: <Playground />,
    errorElement: <ErrorPage error="500" />,
  },
  {
    path: '*',
    element: <ErrorPage />,
    errorElement: <ErrorPage error="500" />,
  },
])

export default function App() {
  const darkMode = usePersist((s) => s.darkMode)

  const theme = React.useMemo(() => {
    const newTheme = createTheme(darkMode ? 'dark' : 'light')
    document.body.style.backgroundColor = newTheme.palette.background.paper
    return newTheme
  }, [darkMode])

  const { location, setStore } = usePersist.getState()
  const { setStatic } = useStatic.getState()

  const [fetched, setFetched] = React.useState<boolean>(false)
  const [error, setError] = React.useState<string>('')

  React.useEffect(() => {
    fetchWrapper<Config>('/config/').then((res) => {
      if (res) {
        if (location[0] === 0 && location[1] === 0) {
          setStore('location', [res.start_lat, res.start_lon])
        }
        setStatic('scannerType', res.scanner_type)
        setStatic('dangerous', res.dangerous || false)
        if (!res.logged_in) {
          router.navigate('/login')
        }
        setFetched(true)
      } else {
        setError('Unable to fetch config, try again later')
      }
    })
  }, [])

  if (!fetched) return null

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <RouterProvider router={router} />
      {error && <ErrorPage error={error} />}
    </ThemeProvider>
  )
}
