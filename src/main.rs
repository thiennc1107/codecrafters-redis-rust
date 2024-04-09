use std::{io::{Error, Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
               match handle_client(_stream) {
                    Ok(..)=> {
                        print!("handle client succesfully");
                    }
                    Err(e) => {
                        print!("failed to handle client err={e}")
                    }

               };
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];
    loop {
        stream.read(&mut  buf)?;
        stream.write_all("+PONG\r\n".as_bytes())?;
    }
}
