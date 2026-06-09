mod stream_utils;

use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};
use stream_utils::{Side, read_stream, write_stream};

// use std::net::SocketAddr;

const MESSAGE_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    let side = Side::Client;
    // println!("[Init] Write Address with Port 0.0.0.0:0");
    // let mut buffer = String::new();
    // let stdin = std::io::stdin();

    // stdin.read_line(&mut buffer)?;

    // let addr: SocketAddr = buffer
    //     .trim()
    //     .parse::<SocketAddr>()
    //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let stream = TcpStream::connect("127.0.0.1:4242")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::ConnectionRefused, e))?;
    let client = Arc::new(Mutex::new(stream));
    println!("- - - {} - - -", &side.to_string().to_uppercase());
    {
        let stream = client.lock().unwrap();
        println!("[{}] Connection: {}", &side, stream.peer_addr()?);
    }

    let mut r_stream = {
        let stream = client.lock().unwrap();
        stream.try_clone()?
    };

    let mut buffer = [0u8; MESSAGE_SIZE];
    let read_data = read_stream(&mut buffer, &mut r_stream, &side)?;
    println!("{}", read_data);

    let stdin = std::io::stdin();
    let mut name_buf = String::new();
    println!("[{}] Write your name (15 char):", side);
    let name_size = stdin.read_line(&mut name_buf)?;

    if name_size > 15 {
        println!("[{}] Name too long", side);

        return Ok(());
    }
    println!(
        "[{}] Send name [{}] to server",
        side,
        &name_buf.trim_ascii_end()
    );
    {
        let mut stream = client.lock().unwrap();
        write_stream(&mut stream, name_buf)?;
    }

    thread::spawn(move || {
        let role = Side::Client;
        loop {
            let message = read_stream(&mut buffer, &mut r_stream, &role).unwrap();
            println!("{}", message);
        }
    });

    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        {
            let mut stream = client.lock().unwrap();
            write_stream(&mut stream, buf)?;
        }
    }
}
