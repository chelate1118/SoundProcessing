use tokio::{net::{TcpSocket, TcpStream}, io::AsyncWriteExt};

pub(crate) struct Kotlin {
    addr: String,
    stream: TcpStream
}

impl Kotlin {
    pub(crate) async fn new() -> Self {
        let port = std::env::args()
                .collect::<Vec<String>>()[1]
                .clone();
        
        let addr = format!("127.0.0.1:{port}");
        
        let stream = TcpStream::connect(&addr).await.unwrap();
        
        Self { addr, stream }
    }
    
    pub(crate) async fn listen(&self) -> Result<(), std::io::Error> {
        loop {
            self.stream.readable().await?;
            
            let mut buf = [0u8; 4096];
            
            match self.stream.try_read(&mut buf) {
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
}