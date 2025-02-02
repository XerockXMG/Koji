/* eslint-disable @typescript-eslint/ban-types */
import type {
  Feature as BaseFeature,
  FeatureCollection as BaseFc,
  GeoJsonProperties,
  GeoJsonTypes,
  Geometry,
  LineString,
  MultiPoint,
  MultiPolygon,
  Point,
  Polygon,
} from 'geojson'

import type { UsePersist } from '@hooks/usePersist'
import type { UseStatic } from '@hooks/useStatic'
import {
  CONVERSION_TYPES,
  PROPERTY_CATEGORIES,
  RDM_FENCES,
  RDM_ROUTES,
  TABS,
  UNOWN_FENCES,
  UNOWN_ROUTES,
} from './constants'

// UTILITY TYPES ==================================================================================

export type SpecificValueType<T, U, V> = {
  [k in keyof T]: T[k] extends U
    ? V extends true
      ? k
      : never
    : V extends true
    ? never
    : k
}[keyof T]

/*
 * OnlyType<T, U, V> - returns a type with only the keys of T that have a value of U
 */
export type OnlyType<T, U, V = true> = { [k in SpecificValueType<T, U, V>]: U }

export type StoreNoFn<T> = keyof OnlyType<T, Function, false>

// ================================================================================================

// GEOJSON TYPES ==================================================================================

export type Properties<G extends Geometry | null = Geometry> =
  GeoJsonProperties & {
    __leafletId?: number
    __forward?: G extends Point ? number : undefined
    __backward?: G extends Point ? number : undefined
    __start?: G extends LineString ? number : undefined
    __end?: G extends LineString ? number : undefined
    __multipoint_id?: G extends Point ? KojiKey : undefined
    __name?: string
    __id?: number
    __geofence_id?: number
    __parent?: number
    __mode?: KojiModes
    __projects?: number[]
    __cells?: string[]
    __index?: number
  }

export interface Feature<G extends Geometry | null = Geometry, P = Properties>
  extends Omit<BaseFeature<G, P>, 'id'> {
  id: G extends Point
    ? number
    : G extends LineString
    ? `${number}__${number}`
    : KojiKey | string
}

export interface FeatureCollection<
  G extends Geometry | null = Geometry,
  P = Properties,
> extends BaseFc<G, P> {
  features: Feature<G, P>[]
}

export type GeometryTypes = Exclude<
  GeoJsonTypes,
  'Feature' | 'FeatureCollection' | 'GeometryCollection'
>

// ================================================================================================

// KOJI TYPES =====================================================================================

export type KojiFenceModes =
  | typeof RDM_FENCES[number]
  | typeof UNOWN_FENCES[number]
export type KojiRouteModes =
  | typeof RDM_ROUTES[number]
  | typeof UNOWN_ROUTES[number]
export type KojiModes = KojiFenceModes | KojiRouteModes | 'unset'

export type KojiSource = 'KOJI' | 'SCANNER' | 'CLIENT'

export type KojiKey = `${number}__${KojiModes}__${KojiSource}`

export type BasicKojiEntry = {
  id: number
  name: string
  created_at: Date | string
  updated_at: Date | string
}

export interface KojiGeofence extends BasicKojiEntry {
  mode: KojiModes
  parent?: number
  geometry: Polygon | MultiPolygon
  geo_type: 'Polygon' | 'MultiPolygon'
}

export interface KojiProperty extends BasicKojiEntry {
  category: typeof PROPERTY_CATEGORIES[number]
  default_value: string | number | boolean | null | object | Array<unknown>
}

export interface KojiGeoProperty
  extends Omit<KojiProperty, 'created_at' | 'updated_at'> {
  value: unknown
  geofence_id: number
  property_id: number
}

export interface KojiProject extends BasicKojiEntry {
  api_endpoint?: string
  api_key?: string
  scanner: boolean
  description?: string
}

export interface KojiRoute extends BasicKojiEntry {
  geofence_id: number
  mode: KojiModes
  description?: string
  geometry: MultiPoint
  points: number
}

export interface KojiTileServer extends BasicKojiEntry {
  url: string
}

export interface AdminGeofence extends KojiGeofence {
  properties: KojiGeoProperty[]
  projects: number[]
  routes: number[]
}

export interface AdminProject extends KojiProject {
  geofences: number[]
}

export interface KojiStats {
  best_clusters: [number, number][]
  best_cluster_point_count: number
  cluster_time: number
  route_time: number
  total_points: number
  points_covered: number
  total_clusters: number
  total_distance: number
  longest_distance: number
  fetch_time: number
  mygod_score: number
}

export interface KojiResponse<T = FeatureCollection> {
  data: T
  status_code: number
  status: string
  message: string
  stats: KojiStats
}

export interface DbOption
  extends Omit<BasicKojiEntry, 'created_at' | 'updated_at'> {
  mode: KojiModes
  geo_type?: GeometryTypes
  geofence_id?: number
  geofences?: number[]
  projects?: KojiProject[]
}

// ================================================================================================

// GENERAL TYPES ==================================================================================

export type TabOption = typeof TABS[number]

export interface Data {
  gyms: PixiMarker[]
  pokestops: PixiMarker[]
  spawnpoints: PixiMarker[]
}

export interface PixiMarker {
  i: `${'p' | 'g' | 'v' | 'u' | 'r'}${number}` & {
    [0]: 'p' | 'g' | 'v' | 'u' | 'r'
  }
  p: [number, number]
}

export interface Config {
  start_lat: number
  start_lon: number
  tile_server: string
  scanner_type: 'rdm' | 'unown' | 'hybrid'
  logged_in: boolean
  dangerous: boolean
}

export type CombinedState = Partial<UsePersist> & Partial<UseStatic>

export type Category = 'pokestop' | 'gym' | 'spawnpoint'

export interface S2Response {
  id: string
  coords: [number, number][]
}

// ================================================================================================

// DATA TYPES =====================================================================================

export type ObjectInput = { lat: number; lon: number }[]
export type MultiObjectInput = ObjectInput[]

export type ArrayInput = number[][]
export type MultiArrayInput = ArrayInput[]

export interface Poracle {
  name?: string
  id?: number
  type?: string
  color?: string
  path?: ArrayInput
  multipath?: MultiArrayInput
  group?: string
  description?: string
  user_selectable?: boolean
  display_in_matches?: boolean
}

export type Conversions =
  | ObjectInput
  | MultiObjectInput
  | ArrayInput
  | MultiArrayInput
  | Geometry
  | Geometry[]
  | Feature
  | Feature[]
  | FeatureCollection
  | string
  | Poracle
  | Poracle[]

export type ConversionOptions = typeof CONVERSION_TYPES[number]
// ================================================================================================

// PROPS ==========================================================================================

export interface PopupProps {
  id: Feature['id']
  properties: Feature['properties']
  dbRef: DbOption | null
}

// ================================================================================================
