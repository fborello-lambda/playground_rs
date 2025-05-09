use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

// Web server from Rust's book:
// https://doc.rust-lang.org/book/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3737")?;
    println!("Starting the WebServer at port 3737");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(3) {
        let stream = stream?;

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", format!("{manifest_dir}/hello.html")),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));

            ("HTTP/1.1 200 OK", format!("{manifest_dir}/hello.html"))
        }
        _ => ("HTTP/1.1 404 NOT FOUND", format!("{manifest_dir}/404.html")),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
