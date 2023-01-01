import * as React from 'react'
import {
  BulkDeleteWithUndoButton,
  Datagrid,
  DeleteWithUndoButton,
  EditButton,
  List,
  NumberField,
  Pagination,
  TextField,
  TopToolbar,
} from 'react-admin'
import { BulkAssignButton } from '../actions/bulk/AssignButton'
import GeofenceCreateButton from './CreateDialog'

function ListActions() {
  return (
    <TopToolbar>
      <GeofenceCreateButton />
    </TopToolbar>
  )
}

const defaultSort = { field: 'id', order: 'ASC' }

function BulkActions() {
  return (
    <>
      <BulkDeleteWithUndoButton resource="geofence" />
      <BulkAssignButton resource="geofence" />
    </>
  )
}

function AreaPagination() {
  return <Pagination rowsPerPageOptions={[25, 50, 100]} />
}

export default function GeofenceList() {
  return (
    <List
      pagination={<AreaPagination />}
      title="Geofences"
      perPage={25}
      actions={<ListActions />}
      sort={defaultSort}
    >
      <Datagrid rowClick="expand" bulkActionButtons={<BulkActions />}>
        <TextField source="name" />
        <NumberField source="related.length" label="Projects" />
        <EditButton />
        <DeleteWithUndoButton />
      </Datagrid>
    </List>
  )
}