//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;

static mut VISITOR_COUNT: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            let splits: ~[&str] = request_str.split(' ').collect();

            let pathString = splits[1].clone();
            let path = Path::new(pathString.slice_from(1).to_owned());

            let pathSplit: ~[&str] = pathString.split('.').collect(); 
            let fileType = pathSplit[pathSplit.len()-1].clone();
            println(fileType);

            let fileToGet = File::open(&path);

            unsafe{
                    VISITOR_COUNT += 1;
                    println!("Received request number: {}", VISITOR_COUNT);
            }

            if(pathString != ~"/") {
                match (fileToGet) {
                    Some(mut file) => {
                        if(fileType == "html") {
                            let msg_bytes: ~[u8] = file.read_to_end();
                            stream.write(msg_bytes);
                            println!("Sending file = {}", pathString);
                        }
                        else {
                            let response: ~str = 
                                 ~"HTTP/1.1 403 Forbidden\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                 <doctype !html>
                                 <html>
                                 <head>
                                    <title>PERMISSION DENIED</title>
                                 </head>
                                 <body>
                                    <h1>You are not allowed to access this file!</h1>
                                 </body></html>\r\n";
                            stream.write(response.as_bytes());
                            println("File was not html, sending 403 Forbidden");
                        }
                    }
                    None => fail!("Error opening file!"),
                }
            }
            else {
                unsafe{

                   let response: ~str = 
                    ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <style>body { background-color: #111; color: #FFEEAA }
                            h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red }
                            h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green }
                     </style></head>
                     <body>
                     <h1>Greetings, Krusty!</h1>
                     <h2>Requests Received: " + VISITOR_COUNT.to_str() + "</h2>
                     </body></html>\r\n"; 
                    stream.write(response.as_bytes());
                    println!("Connection terminates.");
                }
            }
        }
    }
}
