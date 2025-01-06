use std::error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> Result<(), Box<dyn error::Error>> {
    let listener = TcpListener::bind("localhost:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let client_stream = stream?;
        println!("Client Connected");

        thread::spawn(move || {
            if let Err(e) = handle_client(client_stream) {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn error::Error>> {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                eprintln!("Client Disconnected");
                break;
            }
            Ok(n) => {
                let mut buf = &buffer[0..n];
                let mut data = String::new();
                buf.read_to_string(&mut data)?;
                let data = data.trim();
                println!("Received: {data}");

                if let Ok(mut value) = data.parse::<i32>() {
                    value *= 2;
                    let str_value = value.to_string() + "\n";
                    stream.write_all(str_value.as_bytes())?;
                } else {
                    // Echo the buffer
                    stream.write_all(&buffer[0..n])?;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
    Ok(())
}
