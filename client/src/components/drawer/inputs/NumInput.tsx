/* eslint-disable react/jsx-no-duplicate-props */
import * as React from 'react'
import { ListItem, ListItemText, TextField } from '@mui/material'

import { fromCamelCase, fromSnakeCase } from '@services/utils'

interface Props<T> {
  field: T
  value: number | ''
  setValue: (field: T, value: number | '') => void
  disabled?: boolean
  endAdornment?: string
}

export default function NumInput<T extends string>({
  field,
  value,
  setValue,
  endAdornment,
  disabled = false,
}: Props<T>) {
  return (
    <ListItem disabled={disabled}>
      <ListItemText
        primary={
          field.includes('_') ? fromSnakeCase(field) : fromCamelCase(field)
        }
      />
      <TextField
        name={field}
        value={value}
        type="number"
        size="small"
        onChange={({ target }) => {
          setValue(
            field,
            target.value && (+target.value || target.value === '0')
              ? +target.value
              : '',
          )
        }}
        sx={{ width: '35%' }}
        inputProps={{ min: 0, max: 9999 }}
        InputProps={{ endAdornment }}
        disabled={disabled}
      />
    </ListItem>
  )
}