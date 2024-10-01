"use client"

import {
  createContext,
  Dispatch,
  FC,
  ReactNode,
  SetStateAction,
  useContext,
  useState,
} from "react"

export type Language = "Python"

export type PostGradeResponse = {
  problem_id: string
  test_results: TestResult[]
}

export type TestResult = {
  test_id: string
  status: "Passed" | "Failed"
  actual: string
  expected: string
  stderr: string
}

export type ProblemContext = {
  problemId: string
  language: Language
  setLanguage: Dispatch<SetStateAction<Language>>
  code: string
  setCode: Dispatch<SetStateAction<string>>
  grade: PostGradeResponse | undefined
  setGrade: Dispatch<SetStateAction<PostGradeResponse | undefined>>
}

export const ProblemContext = createContext<ProblemContext | undefined>(
  undefined
)

export type ProblemProviderProps = {
  problemId: string
  defaultLanguage: Language
  defaultCode?: string
  children?: ReactNode
}

export const ProblemProvider: FC<ProblemProviderProps> = ({
  problemId,
  defaultLanguage,
  defaultCode = "",
  children,
}) => {
  const [language, setLanguage] = useState(defaultLanguage)
  const [code, setCode] = useState(defaultCode)
  const [grade, setGrade] = useState<PostGradeResponse | undefined>(undefined)

  return (
    <ProblemContext.Provider
      value={{
        problemId,
        language,
        setLanguage,
        code,
        setCode,
        grade,
        setGrade,
      }}
    >
      {children}
    </ProblemContext.Provider>
  )
}

export const useProblem = () => {
  const context = useContext(ProblemContext)
  if (context == null) {
    throw new Error("useProblem must be used within a ProblemProvider")
  }
  return context
}
