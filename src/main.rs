use std::net::{TcpListener, TcpStream};
use std::thread;
// traits
use std::io::Read;
use std::io::Write;
use std::str;
extern crate time;

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 512];
        let m = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    break;
                }
                println!("{}", m);
                m
            }
        };
        let sa = &buf[0..m];

        let s = match str::from_utf8(sa) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("{}", s.starts_with("set "));
        println!("{}", s.trim());
        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();
    thread::spawn(move || memcached_client(&"127.0.0.1:11211"));
    thread::spawn(client);

    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
        }
    }
}

fn memcached_client(addr: &str) {
    let mut stream = TcpStream::connect(&addr).unwrap();

    let b: &[u8] = "set x 0 0 3\r\n123\r\n".as_bytes();
    let _ = stream.write(b);
    let mut buf = [0; 512];
    let m = match stream.read(&mut buf) {
        Err(e) => panic!("Got an error: {}", e),
        Ok(m) => {
            println!("{}", m);
            m
        }
    };
    let sa = &buf[0..m];

    let s = match str::from_utf8(sa) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", s.trim());
}

fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:9527").unwrap();

    let b: &[u8] = "set x 0 0 3\r\n123\r\n".as_bytes();
    let _ = stream.write(b);
    let mut buf = [0; 512];
    let m = match stream.read(&mut buf) {
        Err(e) => panic!("Got an error: {}", e),
        Ok(m) => {
            println!("{}", m);
            m
        }
    };
    let sa = &buf[0..m];

    let s = match str::from_utf8(sa) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", s.trim());
}
