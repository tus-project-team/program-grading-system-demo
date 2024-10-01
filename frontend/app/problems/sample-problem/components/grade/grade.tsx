"use client"

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion"
import type { FC } from "react"
import { useProblem } from "../../contexts/problem"
import { cn } from "@/lib/utils"

export type GradeProps = {
  className?: string
}

export const Grade: FC<GradeProps> = ({ className }) => {
  const { grade } = useProblem()

  return grade == null ? (
    <div />
  ) : (
    <div className={cn("p-4", className)}>
      <h2 className="text-lg font-bold border-b border-separate pb-1">
        Test Results
      </h2>
      <Accordion type="multiple">
        {grade.test_results.map((result) => (
          <AccordionItem value={result.test_id} key={result.test_id}>
            <AccordionTrigger>
              {result.status === "Passed" ? "🟢" : "🔴"} [{result.status}]{" "}
              {result.test_id}
            </AccordionTrigger>
            <AccordionContent>
              <p>
                <span className="font-bold">Expected:</span>
              </p>
              <pre>{result.expected}</pre>
              <p className="mt-2">
                <span className="font-bold">Actual:</span>
              </p>
              <pre>{result.actual}</pre>
              <p className="mt-2">
                <span className="font-bold">Stderr:</span>
              </p>
              <pre>{result.stderr}</pre>
            </AccordionContent>
          </AccordionItem>
        ))}
      </Accordion>
    </div>
  )
}
