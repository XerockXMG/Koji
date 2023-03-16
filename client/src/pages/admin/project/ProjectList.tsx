import { ExportPolygon } from '@components/dialogs/Polygon'
import * as React from 'react'
import {
  BooleanField,
  BulkDeleteWithUndoButton,
  CreateButton,
  Datagrid,
  DeleteWithUndoButton,
  EditButton,
  List,
  NumberField,
  Pagination,
  TextField,
  TopToolbar,
} from 'react-admin'
import { BulkAssignButton } from '../actions/AssignButton'
import { BulkExportButton, ExportButton } from '../actions/Export'

function ListActions() {
  return (
    <TopToolbar>
      <CreateButton />
    </TopToolbar>
  )
}

function BulkActions() {
  return (
    <>
      <BulkDeleteWithUndoButton resource="project" />
      <BulkAssignButton resource="project" />
      <BulkExportButton resource="project" />
    </>
  )
}

export default function ProjectList() {
  return (
    <>
      <List
        pagination={<Pagination rowsPerPageOptions={[25, 50, 100]} />}
        title="Projects"
        actions={<ListActions />}
        perPage={25}
        sort={{ field: 'id', order: 'ASC' }}
      >
        <Datagrid rowClick="expand" bulkActionButtons={<BulkActions />}>
          <TextField source="name" />
          <BooleanField source="scanner" />
          <BooleanField source="api_endpoint" looseValue />
          <BooleanField source="api_key" looseValue />
          <BooleanField source="scanner" />
          <NumberField source="geofences.length" label="Geofences" />
          <EditButton />
          <DeleteWithUndoButton />
          <ExportButton resource="project" />
        </Datagrid>
      </List>
      <ExportPolygon />
    </>
  )
}
