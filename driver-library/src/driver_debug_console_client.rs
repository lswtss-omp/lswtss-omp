const DRIVER_DEBUG_CONSOLE_SERVER_PORT: u32 = 64111;

pub fn init_driver_debug_console_client()
{
    let driver_debug_console_client_socket_handle = std::net::TcpStream::connect(
        format!(
            "127.0.0.1:{}",
            DRIVER_DEBUG_CONSOLE_SERVER_PORT
        ),
    );

    if let Ok(driver_debug_console_client_socket_handle) = driver_debug_console_client_socket_handle
    {
        let _ = simplelog::WriteLogger::init(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            driver_debug_console_client_socket_handle,
        );
    }

    std::panic::set_hook(
        Box::new(
            |panic_info| {
                log::error!("{}", panic_info);
            },
        ),
    );
}
