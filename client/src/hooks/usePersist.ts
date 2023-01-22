import create from 'zustand'
import { persist } from 'zustand/middleware'
import type { FeatureCollection } from 'geojson'
import type { TabOption } from '@assets/types'

export interface UsePersist {
  darkMode: boolean
  tab: number
  drawer: boolean
  location: [number, number]
  zoom: number
  category: 'pokestop' | 'gym' | 'spawnpoint'
  spawnpoint: boolean
  gym: boolean
  pokestop: boolean
  data: 'all' | 'area' | 'bound'
  mode: 'bootstrap' | 'route' | 'cluster'
  radius: number | ''
  min_points: number | ''
  only_unique: boolean
  generations: number | ''
  routing_time: number | ''
  showCircles: boolean
  showLines: boolean
  showPolygons: boolean
  showArrows: boolean
  nativeLeaflet: boolean
  last_seen: Date
  devices: number | ''
  geojson: FeatureCollection
  route_chunk_size: number | ''
  polygonExportMode:
    | 'feature'
    | 'featureCollection'
    | 'featureVec'
    | 'array'
    | 'multiArray'
    | 'struct'
    | 'multiStruct'
    | 'text'
    | 'altText'
    | 'poracle'
  menuItem: TabOption
  export: {
    total: number
    max: number
    route: [number, number][][]
  }
  save_to_db: boolean
  save_to_scanner: boolean
  skipRendering: boolean
  snappable: boolean
  continueDrawing: boolean
  fast: boolean
  loadingScreen: boolean
  simplifyPolygons: boolean
  setActiveMode: 'hover' | 'click'
  setStore: <T extends keyof UsePersist>(key: T, value: UsePersist[T]) => void
}

export const usePersist = create(
  persist<UsePersist>(
    (set) => ({
      darkMode: true,
      tab: 0,
      drawer: false,
      location: [0, 0],
      zoom: 16,
      category: 'pokestop',
      spawnpoint: true,
      gym: true,
      pokestop: true,
      mode: 'cluster',
      data: 'bound',
      radius: 70,
      generations: 1,
      routing_time: 1,
      min_points: 3,
      only_unique: false,
      save_to_db: false,
      save_to_scanner: false,
      skipRendering: false,
      showCircles: true,
      showLines: true,
      showPolygons: true,
      showArrows: true,
      nativeLeaflet: false,
      devices: 1,
      polygonExportMode: 'feature',
      route_chunk_size: 0,
      menuItem: 'Manage',
      fast: false,
      export: {
        total: 0,
        max: 0,
        route: [],
      },
      last_seen: (() => {
        const date = new Date()
        date.setMinutes(0)
        date.setSeconds(0)
        return date
      })(),
      geojson: { type: 'FeatureCollection', features: [] },
      setStore: (key, value) => set({ [key]: value }),
      snappable: true,
      continueDrawing: true,
      loadingScreen: true,
      simplifyPolygons: true,
      setActiveMode: 'hover',
    }),
    {
      name: 'local',
      partialize: (state) =>
        Object.fromEntries(
          Object.entries(state).filter(([key]) => !['export'].includes(key)),
        ) as UsePersist,
    },
  ),
)
