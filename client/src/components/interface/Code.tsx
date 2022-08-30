import * as React from 'react'
import ReactCodeMirror from '@uiw/react-codemirror'
import { json, jsonParseLinter } from '@codemirror/lang-json'
import { linter } from '@codemirror/lint'

interface Props {
  code: string
  setCode: (code: string) => void
  textMode: boolean
}

export function Code({ code, setCode, textMode }: Props) {
  const extensions = React.useMemo(
    () => (textMode ? [json()] : [json(), linter(jsonParseLinter())]),
    [textMode],
  )

  return (
    <ReactCodeMirror
      extensions={extensions}
      theme="light"
      value={code}
      onUpdate={(value) => {
        if (value.docChanged) {
          setCode(value.state.doc.toString())
        }
      }}
    />
  )
}
