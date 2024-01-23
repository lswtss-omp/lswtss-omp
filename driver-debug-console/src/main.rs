use std::io::prelude::BufRead;

const DRIVER_DEBUG_CONSOLE_SERVER_PORT: u32 = 64111;

fn handle_driver_debug_console_client(driver_debug_console_client: std::net::TcpStream)
{
    println!(
        "# New client: {:?}",
        driver_debug_console_client
    );

    let mut driver_debug_console_client_messages_stream_reader_handle =
        std::io::BufReader::new(driver_debug_console_client);

    loop {
        let mut driver_debug_console_client_message = String::new();

        match driver_debug_console_client_messages_stream_reader_handle
            .read_line(&mut driver_debug_console_client_message)
        {
            Ok(_) => print!(
                "{}",
                driver_debug_console_client_message
            ),
            Err(e) => {
                eprintln!(
                    "{}",
                    e
                );
                break;
            },
        }
    }

    println!("# Client terminated");
}

fn main()
{
    let driver_debug_console_server_handle = std::net::TcpListener::bind(
        format!(
            "127.0.0.1:{}",
            DRIVER_DEBUG_CONSOLE_SERVER_PORT
        ),
    )
    .unwrap();

    println!("###########################");
    println!("# LSWTSS Open Modding Platform Driver Debug Console #");
    println!("###########################");
    println!();
    println!(
        "# Server started on port: {}",
        DRIVER_DEBUG_CONSOLE_SERVER_PORT
    );

    for driver_debug_console_client_handle in driver_debug_console_server_handle.incoming() {
        match driver_debug_console_client_handle {
            Ok(stream) => handle_driver_debug_console_client(stream),
            Err(e) => eprintln!(
                "{}",
                e
            ),
        }
    }
}
