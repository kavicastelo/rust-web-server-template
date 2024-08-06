mod thread_pool;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Shutting down server...");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // Get the current directory where the server is running
    let base_path = std::env::current_dir().unwrap();
    let file_path = base_path.join("src").join(filename);
    let contents = fs::read_to_string(file_path)
        .expect(&format!("Failed to read file: {:?}", filename));

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
