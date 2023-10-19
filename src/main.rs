use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:54321").unwrap();

    let pool = ThreadPool::new(4);
    let mut i = 1;
    for stream in listener.incoming() {
        println!("Connection {i} established");
        i += 1;
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down main thread.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line_result = lines.next();
    let request_line: String;
    match request_line_result {
        Some(Ok(line)) => {
            request_line = line;
        }
        Some(Err(e)) => {
            eprintln!("Error reading line: {}", e);
            return;
        }
        None => {
            // Handling Blank Requests
            // Occurrence: Observed in Chrome, not in curl or Firefox.
            // Reason: Chrome preemptively opens multiple TCP connections for potential subsequent requests, even if the user has only requested a single page.
            // These are termed as "speculative connections", "TCP preconnects", leading to "idle sockets" where no data might ever be transmitted.
            // Observation: Chrome opens 2 speculative connections for a request to a site I visit frequently (localhost:7878), and only 1 for a site I visit less frequently (localhost:54321).
            // So it must be tracking my browsing history to determine the number of speculative connections to open.
            // Purpose: Performance optimization. If the user decides to make another request, the TCP handshake delay is avoided.
            // Outcome: If unused, Chrome will close the speculative connection after a certain duration, resulting in an apparent blank request on the server.
            // Additional Info:
            // - To manually close these idle sockets, use: chrome://net-internals/#sockets
            // - https://www.igvita.com/posa/high-performance-networking-in-google-chrome/#tcp-pre-connect
            // - https://bugs.chromium.org/p/chromium/issues/detail?id=116982#c5

            eprintln!("No lines to read from the stream.");
            return;
        }
    }

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
