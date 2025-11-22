use std::{
  env, fs,
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

fn welcome() {
  println!("Now running Eclipse web server!! :)");
  println!(
    "Current directory: {}",
    env::current_dir()
      .expect("Cannot find current dir, panicing.") // i see no reason to handle an error like this if stuff is so fucked up that this errors out.
      .display()
  );
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
  // for some reason on windows i need to read headers to prevent the connection
  // from saying it was closed
  let mut reader = BufReader::new(&mut stream);
  let mut request_line = String::new();

  // read until hit the blank line that terminates headers or EOF.
  loop {
    request_line.clear();
    match reader.read_line(&mut request_line)? {
      0 => break,                           // EOF
      _ if request_line == "\r\n" => break, // end of headers
      _ => continue,
    }
  }

  let body = fs::read_to_string("www/index.html")?;
  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{}",
    body.len(),
    body
  );

  stream.write_all(response.as_bytes())?;
  stream.flush()?;
  Ok(())
}

fn main() -> std::io::Result<()> {
  welcome();

  let listener: TcpListener = TcpListener::bind("127.0.0.1:80")?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        if let Err(err) = handle_client(stream) {
          eprintln!("Client error: {}", err);
        }
      }
      Err(err) => eprintln!("Incoming connection error: {}", err),
    }
  }
  Ok(())
}
