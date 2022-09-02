import React from 'react'
import { useMap, Circle } from 'react-leaflet'

import { ICON_SVG, ICON_RADIUS, ICON_COLOR } from '@assets/constants'
import { useStore } from '@hooks/useStore'
import { getMarkers } from '@services/fetches'
import usePixi from '@hooks/usePixi'
import { useStatic } from '@hooks/useStatic'

export default function Markers() {
  const location = useStore((s) => s.location)
  const data = useStore((s) => s.data)
  const pokestop = useStore((s) => s.pokestop)
  const spawnpoint = useStore((s) => s.spawnpoint)
  const gym = useStore((s) => s.gym)
  const nativeLeaflet = useStore((s) => s.nativeLeaflet)
  const setStore = useStore((s) => s.setStore)

  const selected = useStatic((s) => s.selected)
  const setStatic = useStatic((s) => s.setStatic)
  const pokestops = useStatic((s) => s.pokestops)
  const spawnpoints = useStatic((s) => s.spawnpoints)
  const gyms = useStatic((s) => s.gyms)

  const map = useMap()

  React.useEffect(() => {
    getMarkers(map, data, selected).then((incoming) => {
      setStatic('pokestops', incoming.pokestops)
      setStatic('spawnpoints', incoming.spawnpoints)
      setStatic('gyms', incoming.gyms)
    })
  }, [
    data,
    data === 'area' ? selected : null,
    data === 'bound' ? location : null,
  ])

  const onMove = React.useCallback(() => {
    setStore('location', Object.values(map.getCenter()) as [number, number])
    setStore('zoom', map.getZoom())
  }, [map])

  React.useEffect(() => {
    map.on('moveend', onMove)
    return () => {
      map.off('moveend', onMove)
    }
  }, [map, onMove])

  const initialMarkers = React.useMemo(
    () =>
      [...pokestops, ...spawnpoints, ...gyms]
        .filter(
          (x) =>
            ({ v: spawnpoint, u: spawnpoint, g: gym, p: pokestop }[x.iconId]),
        )
        .map((i) => ({ ...i, customIcon: ICON_SVG[i.iconId] })),
    [
      pokestops.length,
      gyms.length,
      spawnpoints.length,
      pokestop,
      gym,
      spawnpoint,
    ],
  )

  usePixi(nativeLeaflet ? [] : initialMarkers)

  return nativeLeaflet ? (
    <>
      {initialMarkers.map((i) => (
        <Circle
          key={`${i.id}-${i.iconId}`}
          center={i.position}
          radius={ICON_RADIUS[i.iconId]}
          pathOptions={{
            fillOpacity: 100,
            fillColor: ICON_COLOR[i.iconId],
            color: ICON_COLOR[i.iconId],
          }}
        />
      ))}
    </>
  ) : null
}