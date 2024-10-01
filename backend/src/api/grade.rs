use anyhow::Result;
use axum::Json;
use futures::future::join_all;

use crate::utils::Error;

use super::run::{post_run, Input, Language, Output, PostRunPayload, Program};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PostGradeInput {
    problem_id: String,
    language: Language,
    program: Program,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PostGradeOutput {
    problem_id: String,
    test_results: Vec<TestResult>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TestResult {
    test_id: String,
    status: TestStatus,
    actual: String,
    expected: String,
    stderr: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum TestStatus {
    Passed,
    Failed,
}

pub async fn post_grade(
    Json(payload): Json<PostGradeInput>,
) -> Result<Json<PostGradeOutput>, Error> {
    let test_cases = get_test_cases(&payload.problem_id);
    let test_results = join_all(test_cases.iter().map(|test_case| async {
        let Json(output) = post_run(Json(PostRunPayload {
            input: Input {
                stdin: test_case.input.clone(),
            },
            language: payload.language,
            program: Program {
                code: payload.program.code.clone(),
            },
        }))
        .await?;
        let status = if output.stdout == test_case.output {
            TestStatus::Passed
        } else {
            TestStatus::Failed
        };
        Ok(TestResult {
            test_id: test_case.test_id.clone(),
            status,
            actual: output.stdout,
            expected: test_case.output.clone(),
            stderr: output.stderr,
        })
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<TestResult>, Error>>()?;

    Ok(Json(PostGradeOutput {
        problem_id: payload.problem_id,
        test_results,
    }))
}

struct TestCase {
    test_id: String,
    input: String,
    output: String,
}

fn get_test_cases(problem_id: &str) -> Vec<TestCase> {
    if problem_id == "sample-problem" {
        return vec![TestCase {
            test_id: "sample-test-1".to_string(),
            input: "".to_string(),
            output: "Hello, world!\n".to_string(),
        }];
    }

    // TODO: Implement this using a database
    vec![]
}
