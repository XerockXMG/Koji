import React, { Fragment } from 'react'
import { Box, List, Divider } from '@mui/material'
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider'
import type {} from '@mui/x-date-pickers/themeAugmentation'

import { TABS } from '@assets/constants'
import { usePersist } from '@hooks/usePersist'
import { useStatic } from '@hooks/useStatic'
import { useShapes } from '@hooks/useShapes'
import { safeParse } from '@services/utils'

import { Drawer } from '../styled/Drawer'
import DrawerHeader from '../styled/DrawerHeader'
import DrawingTab from './drawing'
import RoutingTab from './routing'
import MenuAccordion from './MenuItem'
import ImportExport from './manage'
import Settings from './settings'
import MiniItem from './MiniItem'
import { Code } from '../Code'

interface Props {
  drawerWidth: number
}

export default function DrawerIndex({ drawerWidth }: Props) {
  const geojson = useStatic((s) => s.geojson)
  const setFromCollection = useShapes((s) => s.setters.setFromCollection)

  const drawer = usePersist((s) => s.drawer)
  const setStore = usePersist((s) => s.setStore)

  const toggleDrawer = (event: React.KeyboardEvent | React.MouseEvent) => {
    if (
      event &&
      event.type === 'keydown' &&
      ((event as React.KeyboardEvent).key === 'Tab' ||
        (event as React.KeyboardEvent).key === 'Shift')
    ) {
      return
    }
    setStore('drawer', false)
  }

  const handleKeyPress = (e: KeyboardEvent) => {
    if (e.code === 'Escape') {
      e.preventDefault()
      setStore('drawer', false)
    }
  }

  React.useEffect(() => {
    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [])

  return (
    <Drawer
      variant="permanent"
      open={drawer}
      drawerWidth={drawerWidth}
      onClose={toggleDrawer}
    >
      {drawer ? (
        <LocalizationProvider dateAdapter={AdapterDayjs}>
          <DrawerHeader>Kōji</DrawerHeader>
          <Divider />
          <List>
            {TABS.map((text, i) => (
              <Fragment key={text}>
                {!!i && <Divider />}
                <MenuAccordion name={text}>
                  {{
                    Drawing: <DrawingTab />,
                    Clustering: <RoutingTab />,
                    Manage: <ImportExport />,
                    Settings: <Settings />,
                    Geojson: (
                      <Code
                        code={JSON.stringify(geojson, null, 2)}
                        setCode={(newCode) => {
                          const parsed = safeParse<typeof geojson>(newCode)
                          if (!parsed.error) {
                            setFromCollection(parsed.value)
                          }
                        }}
                        maxHeight="70vh"
                      />
                    ),
                  }[text] || null}
                </MenuAccordion>
              </Fragment>
            ))}
          </List>
        </LocalizationProvider>
      ) : (
        <Box
          sx={{
            width: '100%',
            height: '100vh',
            display: 'flex',
            alignItems: 'flex-start',
            justifyContent: 'center',
          }}
        >
          <List>
            {TABS.map((text, i) => (
              <MiniItem key={text} text={text} i={i} />
            ))}
          </List>
        </Box>
      )}
    </Drawer>
  )
}
