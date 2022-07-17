# Chapter 20. Final Project: Building a Multithreaded Web Server
- make a web server that says “hello”
- steps
   1. Learn a bit about TCP and HTTP.
   2. Listen for TCP connections on a socket.
   3. Parse a small number of HTTP requests.
   4. Create a proper HTTP response.
   5. Improve the throughput of our server with a thread pool.

## What Did I Learn
- `std::net::TcpListener` listens for incoming tcp connections
- `bind` the `TcpListener` to the provided localhost port 7878
- `unwrap` used to stop the program if errors happen (testing only)
- `incoming` method returns an iterator of tcp `stream` objects
- stream: represents an open connection btwn client and server
- connection: full request and response process
   1. client connects to server
   2. server generates response
   3. server closes connection
- `std::io::prelude` lets us read from/write to the stream
- `for` loop handles looping thru each `stream` in `listener` iterator


- `as_bytes` used on response var to convert string data to bytes
- `unwrap` used again on `stream.write` in case of failure (testing only)
- `flush` halts program exec until all bytes written to connection
- `TcpStream` contains internal buffer, minimizes calls to OS


- `thread pool` group of spawned threads waiting & ready to handle a task
- `thread::spawn` creates a new thread & runs the `{}` code in new thread
```rust
   fn main() {
      let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

      for stream in listener.incoming() {
         let stream = stream.unwrap();

         thread::spawn(|| {
               handle_connection(stream);
         });
      }
   }
```