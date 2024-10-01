"use client"

import MonacoEditor from "@monaco-editor/react"
import { useProblem } from "../../contexts/problem"
import { FC } from "react"
import { cn } from "@/lib/utils"

export type EditorProps = {
  className?: string
}

export const Editor: FC<EditorProps> = ({ className }) => {
  const { language, code, setCode } = useProblem()

  return (
    <MonacoEditor
      className={cn("my-4", className)}
      height="100%"
      language={language}
      value={code}
      onChange={(value) => setCode(value ?? "")}
    />
  )
}
