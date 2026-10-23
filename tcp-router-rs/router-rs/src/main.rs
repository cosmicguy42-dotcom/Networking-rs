use axum::{extract::State, routing::get, Router};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};


async fn handle_msg(State(msg): State<String>) -> String {
    format!("The variable passed was: {}", msg)
}


fn handle_connection(mut stream: TcpStream) -> Option<String> {
    let mut buffer = [0u8; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                return None;
            }

            let msg = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
            Some(msg)
//            println!("Recepted {:?}", msg);
        }
        Err(e) => 
        {
            eprint!("Error: \n{}", e);
            None 
        }
    }

}

fn main() ->  std::io::Result<()> 
{
    let ip = "127.0.0.1:8080";
    let listener = TcpListener::bind(ip)?;

    
    println!("Listening to {}\nWaiting for request...", ip);
    for stream in listener.incoming() 
    {

        match stream {
            Ok(stream) => {
                if let Some(msg) = handle_connection(stream) {
                    
                    let router: Router<()> = Router::new()
                        .route("/", get(handle_msg))
                        .with_state(msg); 
                
                
                } else {
                    eprintln!("Client disconnected or sent empty data.");
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}


/*
        let mut stream = stream?;
        handle_connection(stream);

//         let my_data = String::from(stream?);

         let app = Router::new()
             .route("/", get(handle_msg))
             .with_state(stream?);
    }
    Ok(())
}

*/
