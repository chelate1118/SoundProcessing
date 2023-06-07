use tokio::net::TcpStream;

pub(crate) async fn start_connection() -> Result<(), std::io::Error> {
    let stream = get_stream().await;

    listen(&stream).await
}

async fn get_stream() -> TcpStream {
    TcpStream::connect(&get_address()).await.unwrap()
}

fn get_address() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    let port = &args[1];

    format!("127.0.0.1:{port}")
}

async fn listen(stream: &TcpStream) -> Result<(), std::io::Error> {
    loop {
        stream.readable().await?;

        let mut buf = [0u8; 4096];

        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {n} bytes");
                println!("{}", std::str::from_utf8(&buf).unwrap())
            },
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                println!("wouldblock");
                continue
            }
            Err(e) => {
                return Err(e.into())
            }
        }
    }

    Ok(())
}