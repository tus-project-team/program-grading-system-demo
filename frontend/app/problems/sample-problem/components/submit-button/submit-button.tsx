"use client"

import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import { useState, type FC } from "react"
import { useProblem } from "../../contexts/problem"
import { Loader2Icon } from "lucide-react"

type PostGradeResponse = {
  problem_id: string
  test_results: TestResult[]
}

type TestResult = {
  test_id: string
  status: "Passed" | "Failed"
  actual: string
  expected: string
  stderr: string
}

export type SubmitButtonProps = {
  className?: string
}

export const SubmitButton: FC<SubmitButtonProps> = ({ className }) => {
  const { language, code, setGrade } = useProblem()
  const [submitting, setSubmitting] = useState(false)

  const submit = async () => {
    setSubmitting(true)
    try {
      const res = await fetch("http://localhost:5000/api/v1/grade", {
        method: "POST",
        body: JSON.stringify({
          problem_id: "sample-problem",
          language,
          program: { code },
        }),
        headers: {
          "Content-Type": "application/json",
        },
      })
      const result: PostGradeResponse = await res.json()
      setGrade(result)
    } catch (e) {
      console.error(e)
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <Button
      className={cn(className, "absolute right-4 bottom-4")}
      onClick={submit}
      disabled={submitting}
    >
      {submitting ? (
        <>
          <Loader2Icon className="animate-spin mr-2" />
          <span>実行中です...</span>
        </>
      ) : (
        <span>提出する</span>
      )}
    </Button>
  )
}
