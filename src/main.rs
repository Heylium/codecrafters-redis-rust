
// Uncomment this block to pass the first stage
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, TcpListener};

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    handle_connection(&mut stream).await.unwrap();
                });
            },
            Err(e) => {
                println!("error {}", e);
            }
        }
    }
}

async fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0;512];
    loop {
        let bytes_read = stream.read(&mut buf).await?;
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }
        let res = "+PONG\r\n";
        stream.write(res.as_bytes()).await?;
    }
    Ok(())
}
