use structopt::StructOpt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::blockchain::{Block, Blockchain};

mod blockchain;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "P", long)]
    port: u16,
    #[structopt(short, long = "peer")]
    peers: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let listener = TcpListener::bind(("localhost", opt.port)).await?;

    for peer in opt.peers {
        let mut socket = TcpStream::connect(&peer).await?;
        let mut bc = Blockchain::new();
        bc.anchor(Block::new(&format!("hello from localhost:{}", opt.port)));
        bc.anchor(Block::new("bye"));
        socket.write_all(&serde_json::to_vec(&bc)?).await?;
    }

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut json = String::new();
            match socket.read_to_string(&mut json).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Socket write error: {:?}", err);
                    return;
                }
            }
            println!("Received: {:?}", json);
        });
    }
}
