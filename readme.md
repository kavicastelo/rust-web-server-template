# Simple Rust Web Server

This is a basic web server written in Rust. The server listens for HTTP requests and serves HTML files based on the
requested paths. It includes a feature to handle graceful shutdowns using Ctrl+C.

## Features

- Serves static HTML files
- Supports simple routing with a delay simulation
- Graceful shutdown with Ctrl+C

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system.
- A terminal or command prompt.

### Installation

1. Clone the repository or download the source code.

   ```bash
   git clone https://github.com/kavicastelo/rust_web_server.git
   cd rust_web_server
   ```

2. Build the project using Cargo.

   ```bash
   cargo build
   ```

### Running the Server

1. Start the server by running the following command:

   ```bash
   cargo run
   ```

2. The server will start and listen for incoming HTTP requests.

### Accessing the Server

- Open a web browser or use a tool like `curl` to make a request to the server.
- The server responds to the following routes:
    - `/` - serves the `index.html` file
    - `/sleep` : Simulates a delayed response and then serves the `hello.html` file.
    - Any other path will result in a `404 Not Found` response with the `404.html` file.

### Example Requests

- Access the root path:

   ```bash
   curl http://localhost:7878/
   ```

- Access the `sleep` path:

   ```bash
   curl http://localhost:7878/sleep
   ```

- Access an invalid path to trigger a 404:

   ```bash
   curl http://localhost:7878/other_path
   ```

### Shutting Down the Server

- To stop the server, press `Ctrl+C`. The server will handle the signal and shut down gracefully.

## File Structure

```text
rust_web_server/
├── src/
│   ├── main.rs      # Main server implementation
│   ├── hello.html   # Sample HTML file served at the root path
│   └── 404.html     # HTML file served for 404 responses
├── Cargo.toml       # Cargo configuration file
└── README.md        # This README file
```

### Customization

- You can customize the HTML files by editing `hello.html` and `404.html` in the `src` directory.
- Modify the routing logic in `main.rs` to serve different files or add more routes.

## License

This project is licensed under the MIT license. - see the [LICENSE](LICENSE) file for details.