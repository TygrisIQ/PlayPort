use std::net::{UdpSocket, TcpStream, TcpListener};
use std::sync::mpsc;
use std::thread;
use std::io::{BufRead, BufReader};
pub fn broadcast() -> !{
   
   let socket = UdpSocket::bind("0.0.0.0:8005").unwrap(); 
//    println!("UDP DISCOVERY ON: {}", udp_address); 

   let mut buf = [0; 1024];
   loop{ 
    let (len, source) = socket.recv_from(&mut buf).unwrap();
        let message = String::from_utf8_lossy(&buf[..len]);


        if message.trim() == "IamClient" {
            println!("UDP: client found at {}", source);
            socket.send_to(b"IamServer", source).unwrap();
        }
}}

fn handle_client(stream: TcpStream, tx: mpsc::Sender<Vec<u8>>) {
    let peer = stream.peer_addr().ok();
    println!("Client connected: {:?}", peer);
    let mut reader = BufReader::new(stream);
    let mut buf = Vec::new();
    loop {
        buf.clear();
        match reader.read_until(b'\n', &mut buf) {
            Ok(0) => { println!("Client disconnected: {:?}", peer); break; }
            Ok(_) => {
                // strip newline delimiter
                if buf.last() == Some(&b'\n') { buf.pop(); }
                if !buf.is_empty() {
                    eprintln!("[SERVER] received {} bytes: {:?}", buf.len(), buf);
                    if tx.send(buf.clone()).is_err() { break; }
                }
            }
            Err(e) => { eprintln!("Read error: {}", e); break; }
        }
    }
}
// fn handle_client(mut stream: TcpStream) -> String{  
//         let mut buf = [0; 1024];
//        loop{  
//         match stream.read(&mut buf){
//             Ok(0) => {
//                 println!("connection closed!");
//                 break String::new();
//             }
//             Ok(br) =>{
//                   let received = String::from_utf8_lossy(&buf[..br]);
//                   println!("Received: {}", received);
//                   return received.to_string();       
//             }
//             Err(e) => {
//                 eprintln!("Tcp stream parse error: {}",e);
//             }
//         }

//        }
    

// }
pub fn tcp_listener(port: &str, tx: mpsc::Sender<Vec<u8>>) {    let ip = "0.0.0.0".to_owned() + ":" +&port;
    println!("{}",ip);
    let listener = TcpListener::bind(ip).expect("could not bind to Ip and port");
    println!("local addr: {}",listener.local_addr().unwrap()); 
    println!("TCP listening on {}", listener.local_addr().unwrap());
    //
    //Listen to stream
    for stream in listener.incoming(){
        //hanlde incoming stream
        match stream{
            Ok(stream) => {
                let tx = tx.clone();
                thread::spawn(move || handle_client(stream, tx));
            },
            Err(e) => {
                eprint!("{}",e);
            }
        }
        } 
}
