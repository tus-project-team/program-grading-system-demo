use core::str;

use anyhow::Result;
use axum::Json;
use wasmtime_wasi::{
    pipe::{MemoryInputPipe, MemoryOutputPipe},
    preview1::WasiP1Ctx,
    WasiCtxBuilder,
};

use crate::utils::Error;

const RUST_PYTHON: &[u8] = include_bytes!("../fixtures/wasm/rustpython.wasm");

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PostRunPayload {
    pub language: Language,
    pub program: Program,
    pub input: Input,
}

pub async fn post_run(Json(payload): Json<PostRunPayload>) -> Result<Json<Output>, Error> {
    let output = match payload.language {
        Language::Python => run_python(payload.program, payload.input).await,
    }?;
    Ok(Json(output))
}

pub async fn run_python(program: Program, input: Input) -> Result<Output, Error> {
    let output = run_binary(
        RUST_PYTHON,
        vec!["".to_string(), "-c".to_string(), program.code],
        input,
    )
    .await?;
    Ok(output)
}

pub async fn run_binary(binary: &[u8], args: Vec<String>, input: Input) -> Result<Output, Error> {
    let stdin = input.stdin;

    let engine = wasmtime::Engine::new(wasmtime::Config::new().async_support(true))?;

    let mut linker: wasmtime::Linker<WasiP1Ctx> = wasmtime::Linker::new(&engine);
    wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |t| t)?;

    let stdin_stream = MemoryInputPipe::new(stdin.as_bytes().to_vec());
    let stdout_stream = MemoryOutputPipe::new(1024);
    let stderr_stream = MemoryOutputPipe::new(1024);

    let wasi_ctx = WasiCtxBuilder::new()
        .args(&args)
        .stdin(stdin_stream)
        .stdout(stdout_stream.clone())
        .stderr(stdout_stream.clone())
        .build_p1();

    let mut store = wasmtime::Store::new(&engine, wasi_ctx);

    let module = wasmtime::Module::from_binary(&engine, binary)?;
    let function = linker
        .module_async(&mut store, "", &module)
        .await?
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?;

    function.call_async(&mut store, ()).await?;

    let stdout_bytes = stdout_stream.contents();
    let stdout_str = str::from_utf8(&stdout_bytes)?;

    let stderr_bytes = stderr_stream.contents();
    let stderr_str = str::from_utf8(&stderr_bytes)?;

    Ok(Output {
        stdout: stdout_str.to_string(),
        stderr: stderr_str.to_string(),
    })
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
pub enum Language {
    Python,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Program {
    pub code: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Input {
    pub stdin: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
}
