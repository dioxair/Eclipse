use std::{
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
  // for some reason on windows i need to read headers to prevent the connection
  // from saying it was closed
  let mut reader = BufReader::new(&mut stream);
  let mut request_line = String::new();

  // read until hit the blank line that terminates headers or EOF.
  loop {
    request_line.clear();
    match reader.read_line(&mut request_line) {
      // EOF
      Ok(0) => break,
      // end of headers
      Ok(_) if request_line == "\r\n" => break,
      Ok(_) => continue,
      Err(err) => {
        eprintln!("Failed to read request: {}", err);
        break;
      }
    }
  }

  let body = "OK";
  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\n{}",
    body.len(),
    body
  );

  if let Err(err) = stream.write_all(response.as_bytes()) {
    eprintln!("Failed to write response: {}", err);
    return;
  }
  if let Err(err) = stream.flush() {
    eprintln!("Failed to flush response: {}", err);
  }
}

fn main() -> std::io::Result<()> {
  let listener: TcpListener = TcpListener::bind("127.0.0.1:80")?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => handle_client(stream),
      Err(err) => eprintln!("Incoming connection error: {}", err),
    }
  }
  Ok(())
}
