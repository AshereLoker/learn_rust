use std::{
    fmt,
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ConnectionClosedError;

impl fmt::Display for ConnectionClosedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Сonnection closed")
    }
}

impl std::error::Error for ConnectionClosedError {}

#[allow(dead_code)]
pub enum Side {
    Client,
    Server,
}

#[allow(dead_code)]
pub type SharedStream = Arc<Mutex<TcpStream>>;

#[allow(dead_code)]
pub type Clients = Arc<Mutex<Vec<SharedStream>>>;

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Client => "Client",
            Self::Server => "Server",
        })
    }
}

pub fn read_stream(
    buffer: &mut [u8],
    stream: &mut TcpStream,
    role: &Side,
) -> std::io::Result<String> {
    let bytes_count = stream.read(buffer)?;

    if bytes_count == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            ConnectionClosedError,
        ));
    }

    let bytes = &buffer[..bytes_count];
    let trimmed = bytes.trim_ascii_end();

    match std::str::from_utf8(trimmed) {
        Ok(data) => Ok(data.to_string()),
        Err(e) => {
            println!("[{}] Parse data from server error: {}", role, e);

            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        }
    }
}

pub fn write_steam(stream: &mut TcpStream, message: String) -> std::io::Result<()> {
    stream.write_all(message.as_bytes())?;
    stream.flush()?;

    Ok(())
}

#[allow(dead_code)]
pub fn send_all_exclude(
    client: SharedStream,
    clients: Clients,
    message: &String,
) -> std::io::Result<()> {
    let clients = clients.lock().unwrap();
    for (_, l_client) in clients.iter().enumerate() {
        if !Arc::ptr_eq(&client, l_client) {
            let message = format!("{}", message);
            let mut stream = l_client.lock().unwrap();
            write_steam(&mut stream, message)?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn send_all(clients: Clients, message: &String) -> std::io::Result<()> {
    let clients = clients.lock().unwrap();
    for l_client in clients.iter() {
        let message = format!("{}", message);
        let mut stream = l_client.lock().unwrap();
        write_steam(&mut stream, message)?;
    }

    Ok(())
}
