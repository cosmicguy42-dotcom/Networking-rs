use std::net::{TcpListener, TcpStream};
use std::io::Read;


fn handle_client(mut stream: TcpStream)
{
    let mut buffer = [0u8; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                return;
            }
            println!("Recepted {:?}", String::from_utf8_lossy(&buffer[..bytes_read])); 
        }
        Err(e) => {
            eprintln!("Error: \n{}", e);
        }
    }
}

fn main() -> std::io::Result<()> 
{
    let ip = "127.0.0.1:8080";


    let listener = TcpListener::bind(ip)?; 

    println!("Listening to {}", ip);
    for stream in listener.incoming() 
    {
        handle_client(stream?);
    }

    Ok(())
}
