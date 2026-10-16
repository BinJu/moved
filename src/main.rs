use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value="127.0.0.1:8080")]
    addr: String,
    /// The string pattern to look for
    #[arg(short, long, default_value_t=301)]
    code: u16,

    /// The path to the file to read
    #[arg(short, long)]
    location: String,

    #[arg(short, long, default_value="")]
    message: String
}

fn main() {
    let args = Cli::parse();

    println!("[info] returned http code: '{}'", args.code);
    println!("[info] new location {}", args.location);
    
    // Bind the listener to a local port
    let listener = TcpListener::bind(args.addr.clone()).unwrap();
    println!("Server running on http://{}", args.addr);

    // Listen for incoming TCP connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &args);
    }
}

fn handle_connection(mut stream: TcpStream, args: &Cli) {
    let buf_reader = BufReader::new(&stream);
    
    // Read the first line of the HTTP request (e.g., "GET / HTTP/1.1")
    let request_line_optional= buf_reader.lines().next();
    match request_line_optional {
        Some(request_line_result) => {
            match request_line_result {
                Ok(request_line) => {
                    println!("Received request: {}", request_line);
                    //
                    // Formulate a proper plain-text HTTP response
                    let status_line = format!("HTTP/1.1 {} Moved Permanently", args.code);
                    let location = format!("Location: {}", args.location);
                    let contents = if args.message.is_empty() {format!("This app has to moved to new location: {}", args.location) } else { args.message.clone() };
                    let length = contents.len();

                    let response = format!(
                        "{status_line}\r\n{location}\r\nContent-Length: {length}\r\nContent-Type: text/plain\r\n\r\n{contents}"
                    );

                    // Send the response back to the client
                    stream.write_all(response.as_bytes()).unwrap();
                },
                Err(err) => {
                    println!("Received error: {}", err);
                    stream.write_all("HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html;\r\nContent-Length: 15\r\n\r\nInternal Error.".as_bytes()).unwrap();
                }
            }
        },
        None => {}
    }
}
