use hello::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
  let variation = 0;

  match variation {
    1 => wrap_function("Main A".to_string(), main_a),
    2 => wrap_function("Main B".to_string(), main_b),
    3 => wrap_function("Main C".to_string(), main_c),
    4 => wrap_function("Main D".to_string(), main_d),
    5 => wrap_function("Main E".to_string(), main_e),
    6 => wrap_function("Main F".to_string(), main_f),
    7 => wrap_function("Main G".to_string(), main_g),
    _ => main_g(),
  };
}

fn wrap_function<F>(title: String, f: F) where F: FnOnce() {
  println!("--- {} START ---", title);
  f();
  println!("--- {} END ---", title);
}

fn main_a() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let _stream = stream.unwrap();

    println!("Connection established!");
  }
}

fn main_b() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection_b(stream);
  }
}

fn handle_connection_b(mut stream: TcpStream) {
  let mut buffer = [0; 512];

  stream.read(&mut buffer).unwrap();

  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main_c() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection_c(stream);
  }
}

fn handle_connection_c(mut stream: TcpStream) {
  let mut buffer = [0; 512];

  stream.read(&mut buffer).unwrap();

  let status = "HTTP/1.1 200 OK";
  let headers = "";
  let body = "";
  let response = format!("{}\r\n{}\r\n{}", status, headers, body);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main_d() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection_d(stream);
  }
}

fn handle_connection_d(mut stream: TcpStream) {
  let mut buffer = [0; 512];

  stream.read(&mut buffer).unwrap();

  let status = "HTTP/1.1 200 OK";
  let headers = "";
  let body = fs::read_to_string("hello.html").unwrap();
  let response = format!("{}\r\n{}\r\n{}", status, headers, body);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main_e() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection_e(stream);
  }
}

fn handle_connection_e(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let (status, headers, filename) = if buffer.starts_with(get) {
    ( "HTTP/1.1 200 OK", "", "hello.html", )
  } else {
    ( "HTTP/1.1 404 NOT FOUND", "", "404.html", )
  };

  let body = fs::read_to_string(filename).unwrap();
  let response = format!("{}\r\n{}\r\n{}", status, headers, body);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main_f() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection_f(stream);
  }
}

fn handle_connection_f(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();
  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";
  let (status, headers, filename) = if buffer.starts_with(get) {
    ( "HTTP/1.1 200 OK", "", "hello.html", )
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ( "HTTP/1.1 200 OK", "", "hello.html", )
  } else {
    ( "HTTP/1.1 404 NOT FOUND", "", "404.html", )
  };

  let body = fs::read_to_string(filename).unwrap();
  let response = format!("{}\r\n{}\r\n{}", status, headers, body);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main_g() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(
      || {
        handle_connection_g(stream);
      }
   );
  }
}

fn handle_connection_g(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();
  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";
  let (status, headers, filename) = if buffer.starts_with(get) {
    ( "HTTP/1.1 200 OK", "", "hello.html", )
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ( "HTTP/1.1 200 OK", "", "hello.html", )
  } else {
    ( "HTTP/1.1 404 NOT FOUND", "", "404.html", )
  };

  let body = fs::read_to_string(filename).unwrap();
  let response = format!("{}\r\n{}\r\n{}", status, headers, body);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    assert!(true);
  }
}

