use std::net::{TcpListener, TcpStream};
use std::io::Read;


fn handle_client(mut stream: TcpStream)
{
    let mut txt = String::new();
    let mut txtU8 = [0u8; 1024];

//    match stream.read_to_string(&mut txt) {
    match stream.read(&mut txtU8) {
        Ok(bytes_read) => {
            println!("Read: {} Byte: {:?}", bytes_read, &txtU8[..bytes_read]);
            println!("Recepted: {:?}", String::from_utf8_lossy(&txtU8));
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
