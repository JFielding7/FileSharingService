use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut stream: TcpStream) {
    println!("Listening");
    let mut buffer = [0; 1024];

    loop {
        let bytes = stream.read(&mut buffer).await;

        match bytes {
            Ok(0) => {
                println!("Connection Closed");
                break;
            }
            Ok(_) => {
                println!("{}", std::str::from_utf8(&buffer).unwrap());
            }
            Err(e) => {
                println!("Could not read socket oops")
            }
        }
    }
}

async fn listen_for_connections(listener: &TcpListener) {
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        println!("Connected");

        tokio::spawn(async move {
            handle_connection(stream).await
        });
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();

    listen_for_connections(&listener).await;
}
