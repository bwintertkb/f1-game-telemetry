//! An UDP echo server that just sends back everything that it receives.
//!
//! If you're on Unix you can test this out by in one terminal executing:
//!
//!     cargo run --example echo-udp
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect -- --udp 127.0.0.1:8080
//!
//! Each line you type in to the `nc` terminal should be echo'd back to you!

#![warn(rust_2018_idioms)]
use binread::{self, io::Cursor, BinRead, BinReaderExt};
use f1_game_telemetry::{Telemetry, TelemetryBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;
// struct Server {
//     socket: UdpSocket,
//     buf: Vec<u8>,
//     to_send: Option<(usize, SocketAddr)>,
// }

// impl Server {
//     async fn run(self) -> Result<(), io::Error> {
//         let Server {
//             socket,
//             mut buf,
//             mut to_send,
//         } = self;

//         let mut buf = [0; 1024];
//         loop {
//             let len = sock.recv(&mut buf).await?;
//             println!("{:?} bytes received from {:?}", len, remote_addr);

//             let len = sock.send(&buf[..len]).await?;
//             println!("{:?} bytes sent", len);
//         }
//     }
// }
#[tokio::main] //this is a test
async fn main() {
    let endpoint = "127.0.0.1:30500";
    let mut tel = TelemetryBuilder::new(endpoint.to_owned())
        .add_motion_data()
        .build();
    let mut rec = tel.record().await;
    while let Some(val) = rec.recv().await {
        let val: Value = serde_json::from_str(&val).unwrap();
        println!("RECEIVED: {}", val);
    }

    // let addr = env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| "127.0.0.1:30500".to_string());
    // let socket = UdpSocket::bind(&addr).await?;
    // println!("Listening on: {}", socket.local_addr()?);

    // let mut buf = [0; 1024 * 10];
    // loop {
    //     let len = socket.recv(&mut buf).await?;
    //     //println!("{:?} bytes received from {:?}", len, addr);
    //     let mut reader = Cursor::new(buf.clone());
    //     let val: PacketHeader = reader.read_le().unwrap();
    //     //println!("ID: {:?}", val.m_packetId); //test
    //     if val.packet_id == 7 {
    //         let mut reader = Cursor::new(buf.clone());
    //         let val: PacketHeader = reader.read_le().unwrap();
    //         let mut reader = Cursor::new(buf.clone());
    //         let val2: PacketCarStatusData = reader.read_le().unwrap();
    //         println!("VAL2: {:?}", val2);
    //     }
    //     // let test =  socket.send(&buf[..len]).await?;
    //     // println!("{:?} bytes sent", len);
    // }
    // Ok(())
}
