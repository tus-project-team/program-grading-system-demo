import { Editor } from "./components/editor"
import { Grade } from "./components/grade/grade"
import { SubmitButton } from "./components/submit-button/submit-button"
import { ProblemProvider } from "./contexts/problem"

const Problem = () => {
  return (
    <ProblemProvider problemId="sample-problem" defaultLanguage="Python">
      <div className="grid grid-cols-2 grid-rows-[1fr_max-content] divide-x divide-y">
        <div className="*:my-6 first:*:mt-0 m-6">
          <h1 className="text-3xl font-bold">問. Hello, world!</h1>
          <p>
            標準出力に "Hello, world!" と出力するプログラムを書いてください。
          </p>
        </div>
        <div className="row-span-2">
          <Editor />
        </div>
        <Grade className="col-start-1 row-start-2" />
        <SubmitButton />
      </div>
    </ProblemProvider>
  )
}
export default Problem
