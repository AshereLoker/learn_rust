mod stream_utils;

use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};
use stream_utils::{Clients, SharedStream, Side, read_stream, write_stream};

use crate::stream_utils::{send_all, send_all_exclude};

const MESSAGE_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    let side = Side::Server;
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    // println!("[Init] Write Address with Port 0.0.0.0:0");
    // let mut addr_string = String::new();
    // let stdin = std::io::stdin();

    // stdin.read_line(&mut addr_string)?;

    // let addr: SocketAddr = addr_string
    //     .trim()
    //     .parse::<SocketAddr>()
    //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let listener = TcpListener::bind("127.0.0.1:4242").expect("[Init] Failed to bind");
    println!("- - - {} - - -", &side.to_string().to_uppercase());
    println!("[{}] Listening on 127.0.0.1:4242", &side);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client = Arc::new(Mutex::new(stream));
                {
                    clients.lock().unwrap().push(client.clone());
                }

                let thread_clients = clients.clone();
                let thread_client = client.clone();

                thread::spawn(move || {
                    let thread_result = manage_connection(thread_client, thread_clients);
                    match thread_result {
                        Ok(()) => {}
                        Err(e) => {}
                    }
                });
            }
            Err(e) => {
                println!("[{}] Accept error: {}", side, e);
            }
        }
    }
    Ok(())
}

fn manage_connection(client: SharedStream, clients: Clients) -> std::io::Result<()> {
    let side = Side::Server;
    let welcome_message = format!("[{}] Welcome to RFCM!", side);
    let mut r_stream = {
        let stream = client.lock().unwrap();
        stream.try_clone()?
    };

    {
        let mut stream = client.lock().unwrap();
        write_stream(&mut stream, welcome_message)?;
        println!("[{}] Client connected from {:?}", side, stream.peer_addr());
    }

    let mut buffer = [0u8; MESSAGE_SIZE];
    let name = read_stream(&mut buffer, &mut r_stream, &side)?;
    {
        let message = format!("[{}] User [{}] connection success", side, name);
        send_all(clients.clone(), &message)?;
    }

    println!("[{}] User [{}] connection success", side, name);

    loop {
        let message = read_stream(&mut buffer, &mut r_stream, &side)?;
        let f_message = format!("[{}] {}", name, message);
        send_all_exclude(client.clone(), clients.clone(), &f_message)?;
    }
}
