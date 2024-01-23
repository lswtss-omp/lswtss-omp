use std::net::SocketAddrV4;
use std::os::windows::prelude::AsRawHandle;
use std::os::windows::prelude::FromRawHandle;
use std::sync::Arc;

use winapi::um::consoleapi::AllocConsole;

use crate::*;

#[tokio::main(flavor = "current_thread")]
pub async fn js_engine_thread_main()
{
    let engine_main_module_file_path = std::env::current_dir()
        .unwrap()
        .join("lswtss-omp-main.js");

    std::fs::write(
        &engine_main_module_file_path,
        "",
    )
    .unwrap();

    let engine_main_module_specifier =
        deno_runtime::deno_core::ModuleSpecifier::from_file_path(&engine_main_module_file_path)
            .unwrap();

    // When Steam is starting the game all stdio handles are null which results in Deno crash. To mitigate this, we create a new console window and use handles it provides.

    let mut stdout_handle = deno_runtime::deno_io::STDOUT_HANDLE
        .try_clone()
        .unwrap()
        .as_raw_handle();

    let mut stdin_handle = deno_runtime::deno_io::STDIN_HANDLE
        .try_clone()
        .unwrap()
        .as_raw_handle();

    let mut stderr_handle = deno_runtime::deno_io::STDERR_HANDLE
        .try_clone()
        .unwrap()
        .as_raw_handle();

    if stdout_handle.is_null() | stdin_handle.is_null() | stderr_handle.is_null() {
        unsafe { AllocConsole() };

        stdout_handle =
            unsafe { winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_ERROR_HANDLE) };

        stdin_handle =
            unsafe { winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_INPUT_HANDLE) };

        stderr_handle =
            unsafe { winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE) };
    }

    let engine_deno_inspector_server = Arc::new(
        deno_runtime::inspector_server::InspectorServer::new(
            std::net::SocketAddr::V4(
                SocketAddrV4::new(
                    std::net::Ipv4Addr::new(
                        127, 0, 0, 1,
                    ),
                    JS_ENGINE_INSPECTOR_SERVER_PORT,
                ),
            ),
            "LWSTSS Open Modding Platform Inspector",
        ),
    );

    log::info!(
        "Visit http://127.0.0.1:{}/json to open the debugger.",
        JS_ENGINE_INSPECTOR_SERVER_PORT,
    );

    let mut engine_deno_worker = deno_runtime::worker::MainWorker::bootstrap_from_options(
        engine_main_module_specifier.clone(),
        deno_runtime::permissions::PermissionsContainer::allow_all(),
        deno_runtime::worker::WorkerOptions {
            stdio: deno_runtime::deno_io::Stdio {
                stderr: deno_runtime::deno_io::StdioPipe::File(
                    unsafe { std::fs::File::from_raw_handle(stderr_handle) },
                ),
                stdin: deno_runtime::deno_io::StdioPipe::File(
                    unsafe { std::fs::File::from_raw_handle(stdin_handle) },
                ),
                stdout: deno_runtime::deno_io::StdioPipe::File(
                    unsafe { std::fs::File::from_raw_handle(stdout_handle) },
                ),
            },
            extensions: vec![test_api::test_extension::init_ops_and_esm()],
            bootstrap: deno_runtime::BootstrapOptions::default(),
            maybe_inspector_server: Some(engine_deno_inspector_server.clone()),
            ..Default::default()
        },
    );

    let _ = engine_deno_worker
        .execute_main_module(&engine_main_module_specifier)
        .await;

    loop {
        let _ = engine_deno_worker
            .js_runtime
            .run_event_loop(
                deno_runtime::deno_core::PollEventLoopOptions {
                    wait_for_inspector: false,
                    pump_v8_message_loop: false,
                },
            )
            .await;
    }
}
